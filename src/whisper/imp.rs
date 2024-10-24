use std::sync::{LazyLock, Mutex};
use std::time::Instant;

use byte_slice_cast::AsSliceOf;
use gst::subclass::prelude::*;
use gst::{glib, FlowError};
use gst_audio::subclass::prelude::*;
use gst_base::subclass::base_transform::{GenerateOutputSuccess, PrepareOutputBufferSuccess};

use crate::silero::Silero;

static CAT: LazyLock<gst::DebugCategory> = LazyLock::new(|| {
    gst::DebugCategory::new(
        "whipser",
        gst::DebugColorFlags::empty(),
        Some("Whisper STT"),
    )
});

struct State {
    samples: Vec<i16>,
    silero: Silero,
}

#[derive(Default)]
pub struct Whisper {
    state: Mutex<Option<State>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Whisper {
    const NAME: &'static str = "Whisper";
    type Type = super::Whisper;
    type ParentType = gst_base::BaseTransform;

    // fn new() -> Self {
    //     todo!()
    // }
}

impl Whisper {}

impl ObjectImpl for Whisper {}

impl GstObjectImpl for Whisper {}

impl ElementImpl for Whisper {
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: LazyLock<gst::subclass::ElementMetadata> = LazyLock::new(|| {
            gst::subclass::ElementMetadata::new(
                "Whisper STT",
                "Filter/Effect/Audio",
                "Transcribes audio",
                "Jan Marais <maraisjandre9@gmail.com",
            )
        });
        Some(&*ELEMENT_METADATA)
    }

    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: LazyLock<Vec<gst::PadTemplate>> = LazyLock::new(|| {
            let src_caps = gst::Caps::builder("text/x-raw")
                .field("format", "utf8")
                .build();
            let src_pad_template = gst::PadTemplate::new(
                "src",
                gst::PadDirection::Src,
                gst::PadPresence::Always,
                &src_caps,
            )
            .unwrap();

            let sink_caps = gst_audio::AudioCapsBuilder::new()
                .format(gst_audio::AUDIO_FORMAT_S16)
                // .layout(gst_audio::AudioLayout::NonInterleaved)
                .rate(16_000)
                .channels(1)
                .build();
            let sink_pad_template = gst::PadTemplate::new(
                "sink",
                gst::PadDirection::Sink,
                gst::PadPresence::Always,
                &sink_caps,
            )
            .unwrap();

            vec![src_pad_template, sink_pad_template]
        });
        PAD_TEMPLATES.as_ref()
    }
}

impl BaseTransformImpl for Whisper {
    const MODE: gst_base::subclass::BaseTransformMode =
        gst_base::subclass::BaseTransformMode::NeverInPlace;
    const PASSTHROUGH_ON_SAME_CAPS: bool = false;
    const TRANSFORM_IP_ON_PASSTHROUGH: bool = false;

    fn start(&self) -> Result<(), gst::ErrorMessage> {
        gst::info!(CAT, imp = self, "started ",);
        let silero = Silero::new("./out.onnx").unwrap();
        // let silero = Silero::new("./silero_vad.onnx").unwrap();
        *self.state.lock().unwrap() = Some(State {
            silero,
            samples: vec![],
        });
        Ok(())
    }

    fn transform_caps(
        &self,
        direction: gst::PadDirection,
        _caps: &gst::Caps,
        filter: Option<&gst::Caps>,
    ) -> Option<gst::Caps> {
        // gst::info!(CAT, imp = self, "tfm caps",);
        let mut caps = if direction == gst::PadDirection::Src {
            gst_audio::AudioCapsBuilder::new()
                .format(gst_audio::AUDIO_FORMAT_S16)
                .layout(gst_audio::AudioLayout::NonInterleaved)
                .rate(16_000)
                .channels(1)
                .build()
        } else {
            gst::Caps::builder("text/x-raw")
                .field("format", "utf8")
                .build()
        };
        if let Some(filter) = filter {
            caps = filter.intersect_with_mode(&caps, gst::CapsIntersectMode::First);
        }
        Some(caps)
    }

    fn generate_output(
        &self,
    ) -> Result<gst_base::subclass::base_transform::GenerateOutputSuccess, gst::FlowError> {
        if let Some(b) = self.take_queued_buffer() {
            let buffer_reader = b.as_ref().map_readable().map_err(|_| FlowError::Error)?;
            let samples: &[i16] = buffer_reader.as_slice_of().map_err(|_| FlowError::Error)?;
            let buffer_len = samples.len();
            if buffer_len == 0 {
                return Ok(GenerateOutputSuccess::NoOutput);
            }

            let mut state = self.state.lock().unwrap();
            if let Some(state_inner) = state.as_mut() {
                if (state_inner.samples.len() + buffer_len) < 480 {
                    state_inner.samples.extend_from_slice(samples);
                    Ok(GenerateOutputSuccess::NoOutput)
                } else {
                    let mut prev_samples = state_inner.samples.clone();
                    prev_samples.extend_from_slice(samples);
                    state_inner.samples.clear();
                    // let now = Instant::now();
                    let level = state_inner.silero.calc_level(&prev_samples).unwrap();
                    // let level = 0.51;
                    state_inner.silero.reset();
                    // dbg!("got level in {:?}", now.elapsed());
                    if level > 0.5 {
                        let txt = format!("hello: {level:.2}\n");
                        let mut b = gst::Buffer::with_size(txt.len()).unwrap();
                        b.get_mut()
                            .unwrap()
                            .copy_from_slice(0, txt.as_bytes())
                            .unwrap();
                        Ok(GenerateOutputSuccess::Buffer(b))
                    } else {
                        // dbg!(level);
                        Ok(GenerateOutputSuccess::NoOutput)
                    }
                }
            } else {
                Ok(GenerateOutputSuccess::NoOutput)
            }
        } else {
            gst::info!(CAT, imp = self, "no output",);
            Ok(GenerateOutputSuccess::NoOutput)
        }
    }
}

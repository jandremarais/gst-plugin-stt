use std::sync::LazyLock;

use byte_slice_cast::AsSliceOf;
use gst::subclass::prelude::*;
use gst::{glib, FlowError};
use gst_audio::subclass::prelude::*;
use gst_base::subclass::base_transform::{GenerateOutputSuccess, PrepareOutputBufferSuccess};

static CAT: LazyLock<gst::DebugCategory> = LazyLock::new(|| {
    gst::DebugCategory::new(
        "whipser",
        gst::DebugColorFlags::empty(),
        Some("Whisper STT"),
    )
});

#[derive(Default)]
pub struct Whisper {}

#[glib::object_subclass]
impl ObjectSubclass for Whisper {
    const NAME: &'static str = "Whisper";
    type Type = super::Whisper;
    type ParentType = gst_base::BaseTransform;
}

impl Whisper {
    fn drain(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        todo!()
    }
}

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
            // vec![sink_pad_template, src_pad_template]
        });
        PAD_TEMPLATES.as_ref()
    }

    // fn provide_clock(&self) -> Option<gst::Clock> {
    //     Some(gst::SystemClock::obtain())
    // }
}

impl BaseTransformImpl for Whisper {
    const MODE: gst_base::subclass::BaseTransformMode =
        gst_base::subclass::BaseTransformMode::NeverInPlace;
    const PASSTHROUGH_ON_SAME_CAPS: bool = false;
    const TRANSFORM_IP_ON_PASSTHROUGH: bool = false;

    fn start(&self) -> Result<(), gst::ErrorMessage> {
        gst::info!(CAT, imp = self, "started ",);
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
            gst::info!(CAT, imp = self, "yes output {}", samples.len());
            let b = gst::Buffer::from_slice("hello".as_bytes());
            Ok(GenerateOutputSuccess::Buffer(b))
        } else {
            gst::info!(CAT, imp = self, "no output",);
            Ok(GenerateOutputSuccess::NoOutput)
        }
        // gst::info!(CAT, imp = self, "generate",);
    }

    // fn transform(
    //     &self,
    //     inbuf: &gst::Buffer,
    //     outbuf: &mut gst::BufferRef,
    // ) -> Result<gst::FlowSuccess, gst::FlowError> {
    //     gst::info!(CAT, imp = self, "tfming",);
    //     // Simulating output text for the example
    //     let output_text = "Recognized speech text";

    //     // Write the recognized text to the output buffer
    //     let mut buffer_map = outbuf.map_writable().map_err(|_| gst::FlowError::Error)?;
    //     buffer_map
    //         .as_mut_slice()
    //         .copy_from_slice(output_text.as_bytes());

    //     Ok(gst::FlowSuccess::Ok)
    // }

    // fn query(&self, direction: gst::PadDirection, query: &mut gst::QueryRef) -> bool {

    // }
}

// impl AudioFilterImpl for Whisper {}

use std::sync::LazyLock;

use gst::glib;
use gst::subclass::prelude::*;
use gst_audio::subclass::prelude::*;
use gst_base::subclass::base_transform::GenerateOutputSuccess;

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
    // type ParentType = gst_audio::AudioFilter;
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
            let caps = gst::Caps::builder("text/x-raw")
                .field("format", "utf8")
                .build();
            let src_pad_template = gst::PadTemplate::new(
                "src",
                gst::PadDirection::Src,
                gst::PadPresence::Always,
                &caps,
            )
            .unwrap();

            let caps = gst_audio::AudioCapsBuilder::new()
                .format(gst_audio::AUDIO_FORMAT_S16)
                .layout(gst_audio::AudioLayout::NonInterleaved)
                .rate(16_000)
                .channels(1)
                .build();
            let sink_pad_template = gst::PadTemplate::new(
                "sink",
                gst::PadDirection::Sink,
                gst::PadPresence::Always,
                &caps,
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

    fn transform_caps(
        &self,
        direction: gst::PadDirection,
        _caps: &gst::Caps,
        filter: Option<&gst::Caps>,
    ) -> Option<gst::Caps> {
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
        if let Some(buffer) = self.take_queued_buffer() {
            if buffer.flags().contains(gst::BufferFlags::DISCONT) {
                self.drain()?;
            }
            gst::debug!(
                CAT,
                imp = self,
                "received buffer of size {:?}",
                buffer.size()
            );
        } else {
            gst::debug!(CAT, imp = self, "no buffer in queue");
        }
        Ok(GenerateOutputSuccess::NoOutput)
    }

    // fn query(&self, direction: gst::PadDirection, query: &mut gst::QueryRef) -> bool {

    // }
}

// impl AudioFilterImpl for Whisper {}

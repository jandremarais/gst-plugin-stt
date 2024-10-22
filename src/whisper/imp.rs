use std::sync::LazyLock;

use gst::glib;
use gst::subclass::prelude::*;
use gst_audio::subclass::prelude::*;

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
    type ParentType = gst_audio::AudioFilter;
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
}

impl BaseTransformImpl for Whisper {
    const MODE: gst_base::subclass::BaseTransformMode =
        gst_base::subclass::BaseTransformMode::NeverInPlace;
    const PASSTHROUGH_ON_SAME_CAPS: bool = false;
    const TRANSFORM_IP_ON_PASSTHROUGH: bool = false;
}

impl AudioFilterImpl for Whisper {}

use gst::glib;
use gst::prelude::*;
mod imp;

glib::wrapper! {
    pub struct Whisper(ObjectSubclass<imp::Whisper>) @extends gst_base::BaseTransform, gst::Element, gst::Object;
}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "whisper",
        gst::Rank::NONE,
        Whisper::static_type(),
    )
}

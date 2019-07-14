#[macro_use]
extern crate glib;
#[macro_use]
extern crate gstreamer;

use glib::subclass::types::ObjectSubclass;
use glib::subclass::object::ObjectImpl;
use gstreamer::subclass::element::ElementImpl;
use gstreamer_base::BaseSrc;                            // <----- Change
use gstreamer_base::subclass::base_src::BaseSrcImpl;    // <----- Change

// Struct containing all the element data
struct Life;

// This trait registers our type with the GObject object system and
// provides the entry points for creating a new instance and setting
// up the class data
impl ObjectSubclass for Life {
    const NAME: &'static str = "Life";
    type ParentType = BaseSrc;      // <----- Change
    type Instance = gstreamer::subclass::ElementInstanceStruct<Self>;
    type Class = glib::subclass::simple::ClassStruct<Self>;

    // This macro provides some boilerplate
    glib_object_subclass!();

    // Called when a new instance is to be created. We need to return an instance
    // of our struct here.
    fn new() -> Self {
        Self {}
    }

    // Called exactly once when registering the type. Used for
    // setting up metadata for all instances, e.g. the name and
    // classification and the pad templates with their caps.
    //
    // Actual instances can create pads based on those pad templates
    // with a subset of the caps given here. In case of basetransform,
    // a "src" and "sink" pad template are required here and the base class
    // will automatically instantiate pads for them.
    fn class_init(klass: &mut glib::subclass::simple::ClassStruct<Self>) {
        use gstreamer::subclass::element::ElementClassSubclassExt;

        // Set the element specific metadata. This information is what
        // is visible from gst-inspect-1.0 and can also be programatically
        // retrieved from the gstreamer::Registry after initial registration
        // without having to load the plugin in memory.
        klass.set_metadata(
            "Simple Life Plugin",                   // long name
            "Filter/Effect/Converter/Video",         // classification
            "Just a simple plugin",                  // description
            "Chiu-Hsiang Hsu <wdv4758h@gmail.com>",  // author
        );

        //////////////////////////////
        // VVVVV Change VVVVV
        //////////////////////////////

        let caps = gstreamer::Caps::new_any();
        let src_pad_template = gstreamer::PadTemplate::new(
            "src",
            gstreamer::PadDirection::Src,
            gstreamer::PadPresence::Always,
            &caps,
        ).unwrap();
        klass.add_pad_template(src_pad_template);

        //////////////////////////////
        // ^^^^^ Change ^^^^^
        //////////////////////////////
    }
}

// Implementation of glib::Object virtual methods
impl ObjectImpl for Life {
    // This macro provides some boilerplate.
    glib_object_impl!();
}

// Implementation of gstreamer::Element virtual methods
impl ElementImpl for Life {}


//////////////////////////////
// VVVVV Change VVVVV
//////////////////////////////

impl BaseSrcImpl for Life {
    fn start(&self, element: &BaseSrc) -> Result<(), gstreamer::ErrorMessage> {
        println!("Started");
        Ok(())
    }

    fn fill(
        &self,
        element: &BaseSrc,
        offset: u64,
        _length: u32,
        buffer: &mut gstreamer::BufferRef,
    ) -> Result<gstreamer::FlowSuccess, gstreamer::FlowError> {
        println!("{:?} | {:?} | {:?} | {:?} | 42", element, offset, _length, buffer);
        Ok(gstreamer::FlowSuccess::Ok)
    }

    fn stop(&self, element: &BaseSrc) -> Result<(), gstreamer::ErrorMessage> {
        println!("Stop");
        Ok(())
    }
}

//////////////////////////////
// ^^^^^ Change ^^^^^
//////////////////////////////

// Registers the type for our element, and then registers in GStreamer under
// the name "Life" for being able to instantiate it via e.g.
// gstreamer::ElementFactory::make().
pub fn register(plugin: &gstreamer::Plugin) -> Result<(), glib::BoolError> {
    gstreamer::Element::register(
        Some(plugin),
        "life",
        gstreamer::Rank::None,
        Life::get_type(),
    )
}

// Plugin entry point that should register all elements provided by this plugin,
// and everything else that this plugin might provide (e.g. typefinders or device providers).
fn plugin_init(plugin: &gstreamer::Plugin) -> Result<(), glib::BoolError> {
    register(plugin)?;
    Ok(())
}

// Static plugin metdata that is directly stored in the plugin shared object
// and read by GStreamer upon loading.
// Plugin name, plugin description, plugin entry point function,
// version number of this plugin, license of the plugin, source package name,
// binary package name, origin where it comes from and the date/time of release.
gst_plugin_define!(
    dummysrc,
    "My simple plugin",
    plugin_init,
    "1.0",
    "MIT/X11",
    "gst-plugins-experiment",
    "GStreamer Experiment Plugins (Arch Linux)",
    "https://github.com/wdv4758h",
    "2019-07-14"
);

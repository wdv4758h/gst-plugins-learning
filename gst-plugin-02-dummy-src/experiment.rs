#[macro_use]
extern crate glib;
#[macro_use]
extern crate gstreamer;

use glib::subclass::types::ObjectSubclass;
use glib::subclass::object::ObjectImpl;
use gstreamer::subclass::element::ElementImpl;

// Struct containing all the element data
struct Hello;

// This trait registers our type with the GObject object system and
// provides the entry points for creating a new instance and setting
// up the class data
impl ObjectSubclass for Hello {
    const NAME: &'static str = "Hello";
    type ParentType = gstreamer::Element;
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
            "Simple Hello Plugin",                   // long name
            "Filter/Effect/Converter/Video",         // classification
            "Just a simple plugin",                  // description
            "Chiu-Hsiang Hsu <wdv4758h@gmail.com>",  // author
        );
    }
}

// Implementation of glib::Object virtual methods
impl ObjectImpl for Hello {
    // This macro provides some boilerplate.
    glib_object_impl!();
}

// Implementation of gstreamer::Element virtual methods
impl ElementImpl for Hello {}

// Registers the type for our element, and then registers in GStreamer under
// the name "hello" for being able to instantiate it via e.g.
// gstreamer::ElementFactory::make().
pub fn register(plugin: &gstreamer::Plugin) -> Result<(), glib::BoolError> {
    gstreamer::Element::register(
        Some(plugin),
        "hello",
        gstreamer::Rank::None,
        Hello::get_type(),
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
    experiment,
    "My simple plugin",                             // env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    "1.0",                                          // concat!(env!("CARGO_PKG_VERSION"), "-", env!("COMMIT_ID")),
    "MIT/X11",
    "gst-plugins-experiment",                       // env!("CARGO_PKG_NAME"),
    "GStreamer Experiment Plugins (Arch Linux)",    // env!("CARGO_PKG_NAME"),
    "https://github.com/wdv4758h",                  // env!("CARGO_PKG_REPOSITORY"),
    "2019-07-14"                                    // env!("BUILD_REL_DATE")
);




//     ($name:ident, $description:expr, $plugin_init:ident,
//      $version:expr, $license:expr, $source:expr,
//      $package:expr, $origin:expr, $release_datetime:expr) => { ... };
// Name
// Description
// Filename
// Version
// License
// Source module
// Binary package
// Origin URL


// 1 plugin, N elements
//
//
// gst-inspect-1.0 target/debug/libgstexperiment.so
// env GST_PLUGIN_PATH=target/debug/ gst-inspect-1.0 experiment
// env GST_PLUGIN_PATH=target/debug/ gst-inspect-1.0 hello

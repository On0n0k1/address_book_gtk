use gio::prelude::*;
use gtk::prelude::*;

use gtk::ResponseType;
use glib::value::ToValue;

use std::env::args;

use glib::subclass;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{glib_object_impl, glib_wrapper, glib_object_subclass};


mod imp{
    use super::*;
    use std::cell::RefCell;

    pub struct RowData{
        name: RefCell<Option<String>>,
    }

    // GObject property definitions for our two values
    static PROPERTIES: [subclass::Property; 1] = [
        subclass::Property("name", |name| {
            glib::ParamSpec::string(
                name,
                "Name",
                "Name",
                None, // Default value
                glib::ParamFlags::READWRITE,
            )
        }),
    ];

    // Basic declaration of our type for the GObject type system
    impl ObjectSubclass for RowData {
        const NAME: &'static str = "RowData";
        type ParentType = glib::Object;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib_object_subclass!();

        // Called exactly once before the first instantiation of an instance. This
        // sets up any type-specific things, in this specific case it installs the
        // properties so that GObject knows about their existence and they can be
        // used on instances of our type
        fn class_init(klass: &mut Self::Class) {
            klass.install_properties(&PROPERTIES);
        }

        // Called once at the very beginning of instantiation of each instance and
        // creates the data structure that contains all our state
        fn new() -> Self {
            RowData{
                name: RefCell::new(None),
            }
        }
    }

    // The ObjectImpl trait provides the setters/getters for GObject properties.
    // Here we need to provide the values that are internally stored back to the
    // caller, or store whatever new value the caller is providing.
    //
    // This maps between the GObject properties and our internal storage of the
    // corresponding values of the properties.
    impl ObjectImpl for RowData {
        glib_object_impl!();

        fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
            let prop = &PROPERTIES[id];

            match *prop {
                subclass::Property("name", ..) => {
                    let name = value
                        .get()
                        .expect("type conformity checked by 'object::set_property'");
                        
                    self.name.replace(name);
                }
                _ => unimplemented!(),
            }
        }

        fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
            let prop = &PROPERTIES[id];

            match *prop {
                subclass::Property("name", ..) => Ok(self.name.borrow().to_value()),
                _ => unimplemented!(),
            }
        }
    }

}

// Public part of the RowData type. This behaves like a normal gtk-rs-style GObject
// binding
glib_wrapper! {
    pub struct RowData(Object<subclass::simple::InstanceStruct<imp::RowData>, 
        subclass::simple::ClassStruct<imp::RowData>, 
        RowDataClass>);

    match fn {
        get_type => || imp::RowData::get_type().to_glib(),
    }
}

// Constructor for new instances. This simply calls glib::Object::new() with
// initial values for our two properties and then returns the new instance
impl RowData {
    pub fn new(name: &str) -> RowData {
        glib::Object::new(Self::static_type(), &[("name", &name),])
            .expect("Failed to create row data")
            .downcast()
            .expect("Created row data is of wrong type")
    }
}

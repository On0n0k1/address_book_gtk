use gio::prelude::*;
use gtk::prelude::*;

use gtk::ResponseType;
use glib::value::ToValue;

use std::env::args;

use glib::subclass;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{glib_object_impl, glib_wrapper, glib_object_subclass};



pub mod address;
pub mod email;
pub mod name;
pub mod phone;

mod imp{
    use super::*;
    use std::cell::RefCell;

    pub struct RowData{
        address: RefCell<address::Address>,
        email: RefCell<email::Email>,
        full_name: RefCell<name::Name>,
        // name: RefCell<Option<String>>,
        name: RefCell<String>,
        phone: RefCell<phone::Phone>,
    }

    impl Default for RowData{
        fn default() -> Self {
            RowData{
                address: RefCell::new(address::Address::default()),
                email: RefCell::new(email::Email::default()),
                full_name: RefCell::new(name::Name::default()),
                // name: RefCell::new(Some(String::from("(empty)"))),
                name: RefCell::new(String::from("(empty)")),
                phone: RefCell::new(phone::Phone::default()),
            }
        }
    }

    impl RowData{
        // fn is_valid_name(name: &str) -> bool{

        //     // let address_names = ["address_1", "address_2", "city", "state", "country", "zip"];
        //     // let email_names = ["email"];
        //     // let name_names = ["first_name", "middle_name", "last_name"];
        //     // let phone_names = ["phone"];

        //     let valid_names = 
        //     [
        //         "address_1", "address_2", "city", "state", "country", "zip",
        //         "email",
        //         "first_name", "middle_name", "last_name",
        //         "phone",
        //     ];
        //     valid_names.contains(&name)
        // }
        pub fn update_name(&self){
            let new_name = self.full_name.borrow().get_name();
            self.name.replace(new_name);
        }

        pub fn set_value(&mut self, name: &str, value: String) -> Result<String, String>{
            
            // let address = &mut self.address;
            // let email = &mut self.email;
            // let name = &mut self.name;
            // let phone = &mut self.phone;

            if let Ok(result) = self.address.borrow_mut().set_value(name, value.clone()){
                return Ok(result)
            } else 
            if let Ok(result) = self.email.borrow_mut().set_value(name, value.clone()){
                return Ok(result)
            } else
            if let Ok(result) = self.full_name.borrow_mut().set_value(name, value.clone()){
                return Ok(result)
            } else
            if let Ok(result) = self.phone.borrow_mut().set_value(name, value.clone()) {
                return Ok(result)
            }
            return Err(format!("Invalid address type: {} ", name))
        }

        pub fn get_value(&self, name: &str) -> Result<String, String> {

            if let Ok(result) = self.address.borrow().get_value(name){
                return Ok(result)
            } else 
            if let Ok(result) = self.email.borrow().get_value(name){
                return Ok(result)
            } else
            if let Ok(result) = self.full_name.borrow().get_value(name){
                return Ok(result)
            } else
            if let Ok(result) = self.phone.borrow().get_value(name) {
                return Ok(result)
            }
            return Err(format!("Invalid address type: {} ", name))
        }

        pub fn get_name(&self) -> Result<String, String> {
            Ok(self.full_name.borrow().get_name())
        }


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
        // subclass::Property("count", |name| {
        //     glib::ParamSpec::uint(
        //         name,
        //         "Count",
        //         "Count",
        //         0,
        //         100,
        //         0, // Allowed range and default value
        //         glib::ParamFlags::READWRITE,
        //     )
        // }),
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
            RowData::default()
            // Self {
            //     name: RefCell::new(None),
            //     count: RefCell::new(0),
            // }
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
                    // let visible_name = value
                    //     .get()
                    //     .expect("type conformity checked by `Object::set_property`");
                    self.update_name();
                    // let new_value= value.get().unwrap();
                    // let new_value: String = new_value.unwrap();
                    // self.name.replace(new_value);
                }
                // subclass::Property("count", ..) => {
                //     let count = value
                //         .get_some()
                //         .expect("type conformity checked by `Object::set_property`");
                //     self.count.replace(count);
                // }
                _ => unimplemented!(),
            }
        }

        fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
            let prop = &PROPERTIES[id];

            match *prop {
                subclass::Property("name", ..) => {
                    self.update_name();
                    // self.visible_name.replace()
                    // Ok(self.name.borrow().get_name().unwrap().to_value())
                    // match self.name.borrow().clone(){
                    //     None => {
                    //         return Ok(String::from("(empty)").to_value());
                    //     },
                    //     Some(x) => {
                    //         return Ok(x.to_value())
                    //     },
                    // }

                    return Ok(self.get_name().unwrap().to_value())
                    // Ok(self.name.borrow().clone().to_value())
                },
                // subclass::Property("count", ..) => Ok(self.count.borrow().to_value()),
                _ => unimplemented!(),
            }
        }
    }

}

// Public part of the RowData type. This behaves like a normal gtk-rs-style GObject
// binding
glib_wrapper! {
    pub struct RowData(Object<subclass::simple::InstanceStruct<imp::RowData>, subclass::simple::ClassStruct<imp::RowData>, RowDataClass>);

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
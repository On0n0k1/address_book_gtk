use gio::prelude::*;
use gtk::prelude::*;

use gtk::ResponseType;
use glib::clone;
use glib::{TypedValue, Value};

use std::env::args;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
// use std::cell::RefCell;

// use crate::fields::client::RowData;
use crate::gtk_custom::custom_gobjects::{
    rowdata::RowData,
    entries::Entries,
};


pub struct Row{
    pub entries: Entries,
    // rowdata to append when renewing liststore
    pub rowdata: RowData,
}

impl Row{
    pub fn new(builder: &gtk::Builder) -> Self{
        let rowdata: RowData = RowData::new(&"(empty)");
        let entries: Entries = Entries::new(builder);

        // append the new row to the main list
        // model.append(&rowdata.clone());
        Row{
            rowdata,
            entries,
        }
    }

    pub fn update_name(&self) -> Result<(), String> {
        let get_name = self.entries.get_name();

        match get_name{
            Ok(name) => {
                if let Err(_) = self.rowdata.set_property("name", &name.to_value()) {
                    return Err(String::from("Something went wrong when setting property"));
                };
                return Ok(());
            },
            Err(err) => {
                return Err(err);
            }
        }

    }

    pub fn get(&mut self) -> HashMap<String, String> {
        self.entries.get()
    }

    pub fn load(&mut self) -> Result<(), String> {
        self.entries.load()
    }

    pub fn save(&mut self) -> Result<(), String> {
        self.entries.save()
    }

    pub fn clear(&mut self) -> Result<(), String> {
        self.entries.clear()
    }

    pub fn is_changed(&self) -> Result<bool, String> {
        self.entries.is_changed()
    }

    pub fn is_stored_empty(&self) -> Result<bool, String> {
        self.entries.is_stored_empty()
    }

    pub fn is_entry_empty(&self) -> Result<bool, String> {
        self.entries.is_entry_empty()
    }
}
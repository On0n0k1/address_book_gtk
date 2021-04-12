use std::collections::HashMap;

use gtk::{Button, ListBox, ListStore};


use gio::prelude::*;
use gtk::prelude::*;

use glib::clone;
use gtk::Entry;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct SingleEntry{
    widget: Entry,
    stored: String,
}

impl Clone for SingleEntry{
    fn clone(&self) -> Self {
        let widget: Entry = self.widget.clone();
        let stored: String = self.stored.clone();

        SingleEntry {
            widget,
            stored
        }
    }
}

impl SingleEntry{
    pub fn new(widget: Entry) -> Self{
        SingleEntry{
            widget,
            stored: String::from(""),
        }
    }

    // pub fn load(&mut self, value: &str) -> Result<(), String> {
    pub fn load(&mut self) -> Result<(), String> {
        self.widget.set_text(&self.stored[..]);
        // self.stored = String::from(value);
        
        // unlock widget
        self.widget.set_editable(true);
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), String> {
        let value = self.widget.get_text();
        if !self.widget.get_editable(){
            return Err(String::from("Tried to save when entry widget was locked."));
        }

        self.stored = value.to_string();
        Ok(())
    }

    // In order to retrieve user input, use "save" method first.
    pub fn get_stored(&self) -> Result<String, String>{
        // if !self.widget.get_editable() {
        //     return Err(String::from("Tried to get stored value when widget was locked."));
        // }
        return Ok(self.stored.clone());
    }

    pub fn clear(&mut self) -> Result<(), String> {
        // self.stored = String::from("");
        self.widget.set_text("");

        // lock widget
        self.widget.set_editable(false);
        Ok(())
    }

    pub fn is_changed(&self) -> Result<bool, String> {
        let stored = &(self.stored)[..];
        let checked = self.widget.get_text();

        if !self.widget.get_editable(){
            return Err(String::from("Tried to check if widget is changed when it is locked. This method shouldn't even called in such case."));
        }

        match &(checked.to_string())[..]{
            stored => {
                return Ok(true);
            },
            _ => {
                return Ok(false);
            }
        }
    }

    pub fn is_stored_empty(&self) -> Result<bool, String>{
        if self.stored.len() == 0 {
            return Ok(true);
        }
        Ok(false)
    }
    
    pub fn is_entry_empty(&self) -> Result<bool, String> {
        if self.widget
            .get_text()
            .to_string()
            .len() == 0 
        {
            return Ok(true)
        }

        Ok(false)
    }

}


pub struct Entries{
    entries: HashMap::<String, SingleEntry>,
    keys: Vec<String>,
}

impl Entries{
    pub fn new(model: &gtk::Builder) -> Self{

        let mut entries: HashMap::<String, SingleEntry> = HashMap::new();
        let names =
        [
            "name_first", "name_middle", "name_last",
            "phone", "email",
            "address_1", "address_2", "address_city", "address_state", "address_country", "address_zip"
        ];
        let mut keys: Vec<String> = Vec::with_capacity(names.len());

        let names = names.iter();
        for key in names{
            let widget = model.get_object(*key).unwrap();
            entries.insert(
                String::from(*key),
                SingleEntry::new(widget),
            );
            keys.push(String::from(*key));
        };

        Entries{
            entries,
            keys,
        }
    }

    // Get all values in a hashmap. For things like debugging.
    pub fn get(&mut self) -> HashMap<String, String> {
        let mut new_entries: HashMap<String, String> = 
            HashMap::with_capacity(
                self.keys.len(),
            );
            
        // for (key, entry) in &self.entries{
        //     self.check_valid_key(key).unwrap();
        //     new_entries.insert(
        //         key.clone(),
        //         entry.get_stored().unwrap(),
        //     );
        // }
        for key in self.keys.iter() {
            // Self::check_valid_key(&self, &key)?;
            // self.entries.get(key).unwrap().load()?;
            let new_key = key.clone();
            let new_entry = self.entries.get_mut(&new_key).unwrap().clone();
            new_entries.insert(
                key.clone(),
                new_entry.get_stored().unwrap().clone(),
            );
        }

        new_entries
    }


    fn check_valid_keys(&self, key: &String) -> Result<(), String> {

        if !self.keys.contains(key){
            return Err(format!("Found this string {} in hashmap that shouldn't be in.", key));
        }
        Ok(())
    }

    pub fn load(&mut self) -> Result<(), String> {
        for key in &mut self.keys {
            // Self::check_valid_key(&self, &key)?;
            self.entries.get_mut(key).unwrap().load()?;
        }
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), String> {
        // for (key, entry) in &mut self.entries{
        //     self.check_valid_key(key)?;
        //     self.save()?;
        // }
        for key in self.keys.iter() {
            // Self::check_valid_key(&self, &key)?;
            let new_key = key.clone();
            self.entries.get_mut(&new_key).unwrap().save()?;
        }
        Ok(())

    }

    pub fn clear(&mut self) -> Result<(), String> {
        // for (key, entry) in &mut self.entries{
        //     self.check_valid_key(key)?;
        //     entry.clear();
        // }

        for key in &mut self.keys {
            // Self::check_valid_key(&self, &key)?;
            self.entries.get_mut(&key.clone()).unwrap().clear()?;
        }

        Ok(())
    }

    pub fn is_changed(&self) -> Result<bool, String> {
        // for (key, entry) in &mut self.entries{
        //     self.check_valid_key(key)?;
        //     if entry.is_changed()?{
        //         // one true is enough to assume that there is a change
        //         return Ok(true);
        //     }
        // }
        for key in &self.keys {
            // Self::check_valid_key(&self, &key)?;
            // self.entries.get(key).unwrap().load()?;
            if self.entries.get(key).unwrap().is_changed()?{
                return Ok(true);
            }
        }
        // all false means that there is no change
        Ok(false)
    }

    pub fn is_stored_empty(&self) -> Result<bool, String> {
        // for (key, entry) in &mut self.entries{
        //     // self.check_valid_key(key)?;
        //     if !(entry.is_stored_empty()?){
        //         return Ok(false);
        //     }
        // }
        for key in &self.keys {
            if !self.entries.get(key).unwrap().is_stored_empty()? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn is_entry_empty(&self) -> Result<bool, String> {
        // for (key, entry) in &mut self.entries{
        //     // self.check_valid_key(key)?;
        //     if !(entry.is_entry_empty()?){
        //         return Ok(false);
        //     }
        // }
        
        for key in &self.keys {
            if !self.entries.get(key).unwrap().is_entry_empty()? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    // used for listboxrow creation
    pub fn get_name(&self) -> Result<String, String> {
        let first_name = self.entries.get("name_first").unwrap().get_stored()?;
        let last_name = self.entries.get("name_last").unwrap().get_stored()?;

        match (&last_name[..], &first_name[..]) {
            ("", _) | (_, "") => {
                return Ok(String::from("(empty)"));
            },
            (_, _) => {}
        }

        let mut name: String = String::from("");
        name.push_str(&last_name[..]);
        name.push_str(", ");
        name.push_str(&first_name[..]);
        Ok(name)
    }


}
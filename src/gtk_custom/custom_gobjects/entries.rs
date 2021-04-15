use std::collections::HashMap;

use gtk::prelude::*;

use crate::gtk_custom::custom_gobjects::entry::SingleEntry;


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
        for key in self.keys.iter() {
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
            self.entries.get_mut(key).unwrap().load()?;
        }
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), String> {
        for key in self.keys.iter() {
            let new_key = key.clone();
            self.entries.get_mut(&new_key).unwrap().save()?;
        }
        Ok(())

    }

    pub fn clear(&mut self) -> Result<(), String> {
        for key in &mut self.keys {
            self.entries.get_mut(&key.clone()).unwrap().clear()?;
        }

        Ok(())
    }

    pub fn is_changed(&self) -> Result<bool, String> {
        for key in &self.keys {
            if self.entries.get(key).unwrap().is_changed()?{
                return Ok(true);
            }
        }
        // all false means that there is no change
        Ok(false)
    }

    pub fn is_stored_empty(&self) -> Result<bool, String> {
        for key in &self.keys {
            if !self.entries.get(key).unwrap().is_stored_empty()? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn is_entry_empty(&self) -> Result<bool, String> {        
        for key in &self.keys {
            if !self.entries.get(key).unwrap().is_entry_empty()? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    // used for listboxrow creation
    pub fn get_name(&self) -> Result<Option<String>, String> {
        let first_name = self.entries.get("name_first").unwrap().get_stored()?;
        let last_name = self.entries.get("name_last").unwrap().get_stored()?;

        match (&last_name[..], &first_name[..]) {
            ("", _) | (_, "") => {
                return Ok(Some(String::from("(empty)")));
            },
            (_, _) => {}
        }

        let mut name: String = String::from("");
        name.push_str(&last_name[..]);
        name.push_str(", ");
        name.push_str(&first_name[..]);
        Ok(Some(name))
    }


}
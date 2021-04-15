use gtk::prelude::*;

use gtk::Entry;

//Must derive these traits in order to be used in a hashmap
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

        widget.set_text(&"");
        widget.set_editable(false);
        widget.set_sensitive(false);
        SingleEntry{
            widget,
            stored: String::from(""),
        }
    }

    pub fn load(&mut self) -> Result<(), String> {
        self.widget.set_text(&self.stored[..]);
        
        // unlock widget
        self.widget.set_editable(true);
        self.widget.set_sensitive(true);
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), String> {
        let value = self.widget.get_text();
        if !self.widget.get_editable(){
            return Err(String::from("Tried to save when entry widget was locked."));
        }

        self.stored = value.to_string();
        self.clear()?;
        Ok(())
    }

    // In order to retrieve user input, use "save" method first.
    pub fn get_stored(&self) -> Result<String, String>{
        return Ok(self.stored.clone());
    }

    pub fn clear(&mut self) -> Result<(), String> {
        self.widget.set_text("");

        // lock widget
        self.widget.set_editable(false);
        self.widget.set_sensitive(false);
        Ok(())
    }

    pub fn is_changed(&self) -> Result<bool, String> {
        let stored = &(self.stored)[..];
        let checked = self.widget.get_text();

        // if !self.widget.get_editable(){
        //     return Err(String::from("Tried to check if widget is changed when it is locked. This method shouldn't even called in such case."));
        // }
        if !self.widget.get_editable(){
            return Ok(false);
        }

        // match &(checked.to_string())[..]{
        //     stored => {
        //         return Ok(true);
        //     },
        //     _ => {
        //         return Ok(false);
        //     }
        // }

        return Ok(stored.eq(&checked));
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
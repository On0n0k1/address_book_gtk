use gtk::{Button, ListBox, ListStore};


use gio::prelude::*;
use gtk::prelude::*;

use glib::clone;

pub struct ButtonCancel{
    pub widget: gtk::Button,
}

impl ButtonCancel{
    pub fn new_and_connect(gtk_button: gtk::Button, model: &gio::ListStore, listbox: &gtk::ListBox) -> Self{
        gtk_button.connect_clicked(clone!(@weak model, @weak listbox => move |_| {
            let selected = listbox.get_selected_row();
    
            if let Some(selected) = selected {
                let idx = selected.get_index();
                // model.remove(idx as u32);
                println!("Cancel button {} ", idx);
            }
        }));

        ButtonCancel{
            widget: gtk_button,
        }
    }
}
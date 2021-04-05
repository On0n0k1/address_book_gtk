
use gio::prelude::*;
use gtk::prelude::*;

use glib::clone;


pub struct ButtonEdit{
    pub widget: gtk::Button,
}

impl ButtonEdit{
    fn new_and_connect(gtk_button: gtk::Button, model: &gio::ListStore, listbox: &gtk::ListBox) -> Self{
        gtk_button.connect_clicked(clone!(@weak model, @weak listbox => move |_| {
            let selected = listbox.get_selected_row();
    
            if let Some(selected) = selected {
                let idx = selected.get_index();
                // model.remove(idx as u32);
                println!("Edit button {}", idx);
            }
        }));

        ButtonEdit{
            widget: gtk_button,
        }
    }
}
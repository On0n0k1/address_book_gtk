
use gio::prelude::*;
use gtk::prelude::*;

use glib::clone;

use crate::fields::client::RowData;

pub struct ButtonNew{
    pub widget: gtk::Button,
}


impl ButtonNew{
    fn new_and_connect(gtk_button: gtk::Button, model: &gio::ListStore, listbox: &gtk::ListBox) -> Self{
        gtk_button.connect_clicked(clone!(@weak model, @weak listbox => move |_| {
            // let selected = listbox.get_selected_row();
    
            // if let Some(selected) = selected {
            //     let idx = selected.get_index();
            //     model.remove(idx as u32);
            // }
            model.append(&RowData::new(&"(empty)"));
        }));

        ButtonNew{
            widget: gtk_button,
        }
    }
}

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



pub fn hi() -> String{
    String::from("Hi from listbox")
}

pub struct Row{
    pub entries: Entries,
    // rowdata to append when renewing liststore
    pub rowdata: RowData,
}

impl Row{
    pub fn new(builder: &gtk::Builder, model: &gio::ListStore) -> Self{
        let rowdata: RowData = RowData::new(&"None");
        let entries: Entries = Entries::new(builder);

        // append the new row to the main list
        model.append(&rowdata.clone());
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

pub struct CustomListBox{
    pub builder: gtk::Builder,
    pub widget: gtk::ListBox,
    pub model: gio::ListStore,
    values: std::vec::Vec::<Row>
}


impl CustomListBox{
    pub fn new(builder: gtk::Builder, window: &gtk::Window, viewport: &gtk::Viewport) -> Self{

        // let model = gio::ListStore::new(RowData::static_type());
        let values: std::vec::Vec::<Row> = std::vec::Vec::new();

        let model = gio::ListStore::new(RowData::static_type());
        println!("Hi\n\n\n");
        
        let listbox = gtk::ListBox::new();
        listbox.bind_model(Some(&model),
            clone!(@weak window => @default-panic, move |item| {
            let box_ = gtk::ListBoxRow::new();
            let item = item.downcast_ref::<RowData>().expect("Row data is of wrong type");

            let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

            // Create the label and spin button that shows the two values
            // of the item. We bind the properties for the two values to the
            // corresponding properties of the widgets so that they are automatically
            // updated whenever the item is changing. By specifying SYNC_CREATE the
            // widget will automatically get the initial value of the item set.
            //
            // In case of the spin button the binding is bidirectional, that is any
            // change of value in the spin button will be automatically reflected in
            // the item.
            let label = gtk::Label::new(None);
            item.bind_property("name", &label, "label")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
                .build();
            hbox.pack_start(&label, true, true, 0);

            // let spin_button = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
            // item.bind_property("count", &spin_button, "value")
            //     .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            //     .build();
            // hbox.pack_start(&spin_button, false, false, 0);

            // // When the edit button is clicked, a new modal dialog is created for editing
            // // the corresponding row
            // let edit_button = gtk::Button::with_label("Edit");
            // edit_button.connect_clicked(clone!(@weak window, @strong item => move |_| {
            //     let dialog = gtk::Dialog::with_buttons(Some("Edit Item"), Some(&window), gtk::DialogFlags::MODAL,
            //         &[("Close", ResponseType::Close)]);
            //     dialog.set_default_response(ResponseType::Close);
            //     dialog.connect_response(|dialog, _| dialog.close());

            //     let content_area = dialog.get_content_area();

            //     // Similarly to the label and spin button inside the listbox, the text entry
            //     // and spin button in the edit dialog are connected via property bindings to
            //     // the item. Any changes will be immediately reflected inside the item and
            //     // by the listbox
            //     let entry = gtk::Entry::new();
            //     item.bind_property("name", &entry, "text")
            //         .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            //         .build();

            //     // Activating the entry (enter) will send response `ResponseType::Close` to the dialog
            //     entry.connect_activate(clone!(@weak dialog => move |_| {
            //         dialog.response(ResponseType::Close);
            //     }));
            //     content_area.add(&entry);

            //     let spin_button = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
            //     item.bind_property("count", &spin_button, "value")
            //         .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            //         .build();
            //     content_area.add(&spin_button);

            //     dialog.show_all();
            // }));
            // hbox.pack_start(&edit_button, false, false, 0);

            box_.add(&hbox);

            // When a row is activated (select + enter) we simply emit the clicked
            // signal on the corresponding edit button to open the edit dialog
            // box_.connect_activate(clone!(@weak edit_button => move |_| {
            //     edit_button.emit_clicked();
            // }));

            box_.show_all();
            box_.upcast::<gtk::Widget>()
        }));

        viewport.add(&listbox);

        // let (model, listbox) = (RefCell::new(model), RefCell::new(listbox));

        CustomListBox{
            builder,
            widget: listbox,
            model,
            values,
        }
    
    }

    fn check_valid_index(&self, idx: u32) -> Result<(), String> {
        let (len1, len2) = ( 
            {self.values.len() as u32}, 
            {self.model.get_n_items() as u32},
        );

        if (idx > len1) || (idx > len2) {
            return Err(format!("Index error \n idx: {}\nlen1: {}\nlen2: {}\n ", idx, len1, len2));
        }
        Ok(())
    }

    pub fn update_names(&self) -> Result<(), String>{
        for rows in &self.values{
            rows.update_name()?;
        }
        Ok(())
    }

    pub fn button_new(&mut self) -> Result<(), String> {
        let new_row = Row::new(&self.builder, &self.model);
        // self.model.append(&new_row.rowdata);
        self.values.push(new_row);
        Ok(())
    }

    pub fn button_delete(&mut self) -> Result<(), String> {
        let selected = self.widget.get_selected_row();
    
        if let Some(selected) = selected {
            let idx = selected.get_index();

            if (idx as usize > self.values.len()) || (idx as usize > self.model.get_n_items() as usize) {
                return Err(format!("out of bounds index. idx: {}, values: {}, model: {}\n", 
                    idx, 
                    self.values.len(), 
                    self.model.get_n_items())
                );
            }

            self.values.swap_remove(idx as usize);
            self.model.remove(idx as u32);
        }
        Ok(())
    }

    pub fn button_edit(&mut self) -> Result<(), String>{
        let selected = self.widget.get_selected_row();
    
        if let Some(selected) = selected {
            let idx = selected.get_index() as usize;

            return self.values[idx].load();
            // if let Err(err) = self.values[idx].load() {
            //     eprintln!("{}", err);
            // }
        }

        Ok(())
    }

    pub fn button_save(&mut self) {
        let selected = self.widget.get_selected_row();

        if let Some(selected) = selected {
            let idx = selected.get_index() as usize;
            
            if let Err(err) = self.values[idx].save() {
                eprintln!("{}", err);
            }
        }
    }

    pub fn button_cancel(&mut self) {
        let selected = self.widget.get_selected_row();

        if let Some(selected) = selected {
            let idx = selected.get_index() as usize;
            
            if let Err(err) = self.values[idx].clear() {
                eprintln!("{}", err);
            }
        }
    }




    // search about connect_row_selected

    // pub fn edit(&self) -> Option<RowData>{

    // }

    // pub fn delete(&mut self) -> Result<RowData, String>{
    //     let selected = self.model.get_selected_row();
    //     if let Some(selected) = selected{
    //         let idx = selected.get_index();

    //         self.model.remove(idx);
    //     }
    // }
}
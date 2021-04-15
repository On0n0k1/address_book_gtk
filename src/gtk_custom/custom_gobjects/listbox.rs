use gio::prelude::*;
use gtk::prelude::*;

use glib::{clone, send_unique::Ref};
use glib::{TypedValue, Value};

// use std::env::args;
// use std::cell::RefCell;
use std::{
    rc::Rc,
    rc::Weak,
    cell::RefCell,
    collections::HashMap,
    env::args,
};
// use std::collections::HashMap;
// use std::cell::RefCell;

// use crate::fields::client::RowData;
use crate::gtk_custom::last_edited::LastEdit;
use crate::gtk_custom::custom_gobjects::{
    rowdata::RowData,
    row::Row,
    entries::Entries,
};



pub struct CustomListBox{
    pub builder: gtk::Builder,
    // window: gtk::Window,
    pub widget: gtk::ListBox,
    pub model: gio::ListStore,
    pub values: Rc<RefCell<std::vec::Vec::<Row>>>,
    pub last_edited: LastEdit,
    // pub last_edited: i32,
}


impl CustomListBox{
    pub fn new(builder: gtk::Builder, viewport: &gtk::Viewport) -> Self{

        // let model = gio::ListStore::new(RowData::static_type());
        let values: Rc<RefCell<std::vec::Vec::<Row>>> = Rc::new(RefCell::new(std::vec::Vec::new()));

        let model = gio::ListStore::new(RowData::static_type());
        let window: gtk::Window = builder.get_object("Main_window").unwrap();
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

            box_.add(&hbox);

            box_.show_all();
            box_.upcast::<gtk::Widget>()
        }));

        viewport.add(&listbox);


        CustomListBox{
            builder,
            widget: listbox,
            model,
            values,
            last_edited: LastEdit::NONE,
        }
    
    }

    pub fn update_names(&self) -> Result<(), String>{
        let length = self.values.borrow().len();
        for idx in 0..length{
            self.values.borrow_mut()[idx].update_name()?;
        }
        Ok(())
    }

    pub fn button_new(&mut self) -> Result<(), String> {
        let new_row = Row::new(&self.builder);
        self.model.append(&new_row.rowdata);
        self.values.borrow_mut().push(new_row);

        self.widget.unselect_all();
        let last_index = self.model.get_n_items() as i32 - 1;

        let last_widget = self.widget.get_row_at_index(last_index).unwrap();
        self.widget.select_row(Some(&last_widget));
        // self.last_edited = last_index;
        self.update_names()?;
        
        self.button_edit()?;
        self.last_edited = LastEdit::NEW(last_index as usize);

        Ok(())
    }

    pub fn delete_from_index(&mut self, idx: i32) -> Result<(), String> {
        if idx < 0 {
            return Ok(());
        }

        let idx = idx as usize;

        if (idx as usize > self.values.borrow_mut().len()) || (idx > self.model.get_n_items() as usize) {
            return Err(format!("out of bounds index. idx: {}, values: {}, model: {}\n", 
                idx, 
                self.values.borrow_mut().len(), 
                self.model.get_n_items())
            );
        }

        self.values.borrow_mut().swap_remove(idx);
        self.model.remove(idx as u32);
        self.last_edited = LastEdit::NONE;
        self.update_names()?;
        Ok(())
    }

    pub fn button_delete(&mut self) -> Result<(), String> {
        let selected = self.widget.get_selected_row();
    
        if let Some(selected) = selected {
            let idx = selected.get_index();

            self.delete_from_index(idx)?;
            self.widget.unselect_all();
        }
        Ok(())
    }

    pub fn button_edit(&mut self) -> Result<(), String>{
        let selected = self.widget.get_selected_row();
    
        if let Some(selected) = selected {
            // self.confirm_change_message()?;
            let idx = selected.get_index() as usize;
            // self.last_edited = idx as i32;
            // self.last_edited = LastEdit::UNSAVED(idx as usize);
            // self.values.borrow_mut()[idx].load().unwrap();
            self.last_edited = LastEdit::UNSAVED(idx);
            self.edit_from_index(idx)?;
        }

        Ok(())
    }

    pub fn edit_from_index(&mut self, idx: usize) -> Result<(), String> {
        // get a reference to the row idx
        let last_widget = self.widget.get_row_at_index(idx as i32).unwrap();
        // set that row as selected
        self.widget.select_row(Some(&last_widget));
        // load it's values to be edited
        self.update_names()?;
        return self.values.borrow_mut()[idx].load();
    }

    // save should just save on top of the last edited, and not allow it to not have invalid entries
    pub fn button_save(&mut self) -> Result<(), String>{
        // If no row was accessed since the last button-press
        match self.last_edited{
            LastEdit::NONE => {
                return self.update_names();
            },
            LastEdit::NEW(idx) => {
                self.last_edited = LastEdit::NONE;
                self.values.borrow_mut()[idx].entries.save()?;
                return self.update_names();
            },
            LastEdit::UNSAVED(idx) => {
                self.last_edited = LastEdit::NONE;
                self.values.borrow_mut()[idx].entries.save()?;
                return self.update_names();
            }
        }
        // if self.last_edited < 0 {
        //     return Ok(());
        // }

        // let idx = self.last_edited as usize;

        // self.last_edited = -1;

        // // If the last row accessed was edited and not saved
        // if !(self.values.borrow_mut()[idx].entries.is_changed()?){
        //     return Ok(())
        // }

    }

    pub fn button_cancel(&mut self) -> Result<(), String>{
        Entries::new(&self.builder).clear()?;
        self.last_edited = LastEdit::NONE;
        return self.update_names();
    }



    
}
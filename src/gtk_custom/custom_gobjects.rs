mod listbox;
mod rowdata;

// mod button_new;
// mod button_delete;
// mod button_edit;
// mod button_cancel;
// mod button_save;

mod entries;
// mod entry_address;
// mod entry_email;
// mod entry_name;
// mod entry_phone;


use std::rc::{Rc, Weak};
use std::cell::RefCell;


use gio::ListStore;
use gtk::prelude::*;
use gtk::Grid;

pub use listbox::CustomListBox;
// pub use button_new::ButtonNew;
// pub use button_delete::ButtonDelete;
// pub use button_edit::ButtonEdit;
// pub use button_cancel::ButtonCancel;
// pub use button_save::ButtonSave;

// pub use entry_address::EntryAddress;
// pub use entry_email::EntryEmail;
// pub use entry_name::EntryName;
// pub use entry_phone::EntryPhone;


pub struct CustomGobjects{
    listbox: Rc<RefCell<CustomListBox>>,
    // button_new: ButtonNew,
    // button_delete: ButtonDelete,
    // button_edit: ButtonEdit,
    // button_cancel: ButtonCancel,
    // button_save: ButtonSave,

    button_new: gtk::Button,
    button_delete: gtk::Button,
    button_edit: gtk::Button,
    button_cancel: gtk::Button,
    button_save: gtk::Button,

    // entry_address: EntryAddress,
    // entry_email: EntryEmail,
    // entry_name: EntryName,
    // entry_phone: EntryPhone,
}


impl CustomGobjects{
    pub fn new_and_connect(window: &gtk::Window, builder: gtk::Builder) -> Self{
        // let builder = Rc::new(RefCell::new(builder));
        let viewport_for_listbox: gtk::Viewport = builder.get_object("listbox_viewport").unwrap();
                
        let listbox = CustomListBox::new(builder, window, &viewport_for_listbox);
        let listbox = RefCell::new(listbox);

        // let inputs: (&gtk::Builder, &CustomListBox) = (, &listbox);
        
        // viewport_for_listbox.add(&listbox.borrow().widget);

        let listbox = Rc::new(listbox);
        
        // let button_cancel: ButtonCancel = ButtonCancel::new_and_connect(inputs.clone());
        // let button_delete: ButtonDelete = ButtonDelete::new_and_connect(inputs.clone());
        // let button_edit: ButtonEdit = ButtonEdit::new_and_connect(inputs.clone());
        // let button_new: ButtonNew = ButtonNew::new_and_connect(inputs.clone());
        // let button_save: ButtonSave = ButtonSave::new_and_connect(inputs.clone());

        // let listbox_ref = (listbox);
        // let button_cancel: ButtonCancel = ButtonCancel::new_and_connect(Rc::downgrade(&listbox));
        // let button_delete: ButtonDelete = ButtonDelete::new_and_connect(Rc::downgrade(&listbox));
        // let button_edit: ButtonEdit = ButtonEdit::new_and_connect(Rc::downgrade(&listbox));
        // let button_new: ButtonNew = ButtonNew::new_and_connect(Rc::downgrade(&listbox));
        // let button_save: ButtonSave = ButtonSave::new_and_connect(Rc::downgrade(&listbox));


        let button_cancel: gtk::Button = Self::get_button_cancel(Rc::downgrade(&listbox));
        let button_delete: gtk::Button = Self::get_button_delete(Rc::downgrade(&listbox));
        let button_edit: gtk::Button = Self::get_button_edit(Rc::downgrade(&listbox));
        let button_new: gtk::Button = Self::get_button_new(Rc::downgrade(&listbox));
        let button_save: gtk::Button = Self::get_button_save(Rc::downgrade(&listbox));

        // let entry_address: EntryAddress = EntryAddress::new_and_connect(inputs.clone());
        // let entry_email: EntryEmail = EntryEmail::new_and_connect(inputs.clone());
        // let entry_name: EntryName = EntryName::new_and_connect(inputs.clone());
        // let entry_phone: EntryPhone = EntryPhone::new_and_connect(inputs.clone());

        
        
        CustomGobjects{
            listbox,
            button_new,
            button_delete,
            button_edit,
            button_cancel,
            button_save,

            // entry_address,
            // entry_email,
            // entry_name,
            // entry_phone,
        }
    
    }

    fn get_button_cancel(listbox: Weak<RefCell<CustomListBox>>) -> gtk::Button {
        let button: gtk::Button = listbox.upgrade().unwrap().borrow_mut().builder.get_object("button_cancel").unwrap();
        button.connect_clicked(move |_| {
            listbox.upgrade().unwrap().borrow_mut().button_cancel();
        });
        button
    }

    fn get_button_delete(listbox: Weak<RefCell<CustomListBox>>) -> gtk::Button {
        let button: gtk::Button = listbox.upgrade().unwrap().borrow_mut().builder.get_object("button_delete").unwrap();
        button.connect_clicked(move |_| {
            listbox.upgrade().unwrap().borrow_mut().button_delete().unwrap();
        });
        button
    }

    fn get_button_edit(listbox: Weak<RefCell<CustomListBox>>) -> gtk::Button {
        let button: gtk::Button = listbox.upgrade().unwrap().borrow_mut().builder.get_object("button_edit").unwrap();
        button.connect_clicked(move |_| {
            listbox.upgrade().unwrap().borrow_mut().button_edit().unwrap();
        });
        button
    }

    fn get_button_new(listbox: Weak<RefCell<CustomListBox>>) -> gtk::Button {
        let button: gtk::Button = listbox.upgrade().unwrap().borrow_mut().builder.get_object("button_new").unwrap();
        button.connect_clicked(move |_| {
            listbox.upgrade().unwrap().borrow_mut().button_new().unwrap();
        });
        button
    }

    fn get_button_save(listbox: Weak<RefCell<CustomListBox>>) -> gtk::Button {
        let button: gtk::Button = listbox.upgrade().unwrap().borrow_mut().builder.get_object("button_save").unwrap();
        button.connect_clicked(move |_| {
            listbox.upgrade().unwrap().borrow_mut().button_save();
        });
        button
    }
}
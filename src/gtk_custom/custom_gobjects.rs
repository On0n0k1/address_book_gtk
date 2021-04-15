pub mod listbox;
pub mod entries;
pub mod entry;
pub mod row;
pub mod rowdata;
pub mod button_behavior;

use std::rc::{Rc, Weak};
use std::cell::RefCell;


use gio::ListStore;
use gtk::prelude::*;
use gtk::Grid;
use gtk::ResponseType;

pub use listbox::CustomListBox;
use button_behavior::{ButtonBehavior, CustomButton};
pub use crate::gtk_custom::{
    do_once::DoOnce,
    last_edited::LastEdit,
};



pub struct CustomGobjects{
    pub listbox: Rc<RefCell<CustomListBox>>,

    // button_new: gtk::Button,
    // button_delete: gtk::Button,
    // button_edit: gtk::Button,
    // button_cancel: gtk::Button,
    // button_save: gtk::Button,

    button_new: CustomButton,
    button_delete: CustomButton,
    button_edit: CustomButton,
    button_cancel: CustomButton,
    button_save: CustomButton,

}


impl CustomGobjects{
    pub fn new_and_connect(builder: gtk::Builder) -> Self{
        // let builder = Rc::new(RefCell::new(builder));
        let viewport_for_listbox: gtk::Viewport = builder.get_object("listbox_viewport").unwrap();
                
        let window: gtk::Window = builder.get_object("Main_window").unwrap();
        let listbox = CustomListBox::new(builder, &viewport_for_listbox);
        let listbox = RefCell::new(listbox);

        let listbox = Rc::new(listbox);

        let button_cancel = CustomButton::new(
            Rc::downgrade(&listbox),
            &"button_cancel",
            ButtonBehavior::CANCEL(Rc::downgrade(&listbox)),
        );

        let button_delete = CustomButton::new(
            Rc::downgrade(&listbox),
            &"button_delete",
            ButtonBehavior::DELETE(Rc::downgrade(&listbox)),
        );

        let button_edit = CustomButton::new(
            Rc::downgrade(&listbox),
            &"button_edit",
            ButtonBehavior::EDIT(Rc::downgrade(&listbox)),
        );

        let button_new = CustomButton::new(
            Rc::downgrade(&listbox),
            &"button_new",
            ButtonBehavior::NEW(Rc::downgrade(&listbox)),
        );

        let button_save = CustomButton::new(
            Rc::downgrade(&listbox),
            &"button_save",
            ButtonBehavior::SAVE(Rc::downgrade(&listbox)),
        );

        // let button_cancel: gtk::Button = Self::get_button_cancel(Rc::downgrade(&listbox));
        // let button_delete: gtk::Button = Self::get_button_delete(Rc::downgrade(&listbox));
        // let button_edit: gtk::Button = Self::get_button_edit(Rc::downgrade(&listbox));
        // let button_new: gtk::Button = Self::get_button_new(Rc::downgrade(&listbox));
        // let button_save: gtk::Button = Self::get_button_save(Rc::downgrade(&listbox));
        

        CustomGobjects{
            listbox,
            button_new,
            button_delete,
            button_edit,
            button_cancel,
            button_save,
        }
    }

    // fn get_button_cancel(listbox: Weak<RefCell<CustomListBox>>) -> gtk::Button {
    //     let button: gtk::Button = listbox.upgrade().unwrap().borrow_mut().builder.get_object("button_cancel").unwrap();
    //     let window: gtk::Window = listbox.upgrade().unwrap().borrow_mut().builder.get_object("Main_window").unwrap();
    //     button.connect_clicked(move |_| {
    //         if let Some(idx) = Self::confirm_change(Weak::clone(&listbox)).unwrap() {
    //             // If the previous edit had change, do normal button behaviour after new window has closed.
    //             let dialog = Self::get_change_message(&window);
    //             let listbox_ref = Weak::clone(&listbox);

    //             dialog.connect_response(move |dialog, resp| {        
    //                 match resp{
    //                     ResponseType::Ok => {
    //                         listbox_ref.upgrade().unwrap().borrow_mut().values.borrow_mut()[idx].save().unwrap();
    //                         // listbox.upgrade().unwrap().borrow_mut().values.borrow_mut()[idx].clear().unwrap();
    //                     },
    //                     ResponseType::Cancel | ResponseType::DeleteEvent => {
    //                         // listbox.upgrade().unwrap().borrow_mut().values.borrow_mut()[idx].clear().unwrap();
    //                     },
    //                     x => println!("{}", x),
    //                 }
        
    //                 // Normal behaviour here
    //                 listbox_ref.upgrade().unwrap().borrow_mut().button_cancel().unwrap();


    //                 dialog.close();
    //             });
    //             dialog.show_all();
    //         }else{
    //             // If the previous edit wasnt changed, do normal button behaviour.
    //             listbox.upgrade().unwrap().borrow_mut().button_cancel().unwrap();
    //         }
    //     });
    //     button
    // }

    // fn get_button_delete(listbox: Weak<RefCell<CustomListBox>>) -> gtk::Button {
    //     let button: gtk::Button = listbox.upgrade().unwrap().borrow_mut().builder.get_object("button_delete").unwrap();
    //     let window: gtk::Window = listbox.upgrade().unwrap().borrow_mut().builder.get_object("Main_window").unwrap();
    //     button.connect_clicked(move |_| {
    //         // Self::confirm_change_message(Weak::clone(&listbox), window.clone()).unwrap();
    //         // listbox.upgrade().unwrap().borrow_mut().button_delete().unwrap();

    //         if let Some(idx) = Self::confirm_change(Weak::clone(&listbox)).unwrap() {
    //             // If the previous edit had change, do normal button behaviour after new window has closed.
    //             let dialog = Self::get_change_message(&window);

    //             let listbox_ref = Weak::clone(&listbox);

    //             dialog.connect_response(move |dialog, resp| {        
    //                 match resp{
    //                     ResponseType::Ok => {
    //                         listbox_ref.upgrade().unwrap().borrow_mut().values.borrow_mut()[idx].save().unwrap();
    //                         // listbox.upgrade().unwrap().borrow_mut().values.borrow_mut()[idx].clear().unwrap();
    //                     },
    //                     ResponseType::Cancel | ResponseType::DeleteEvent => {
    //                         // listbox.upgrade().unwrap().borrow_mut().values.borrow_mut()[idx].clear().unwrap();
    //                     },
    //                     x => println!("{}", x),
    //                 }
        
    //                 // Normal behaviour here
    //                 listbox_ref.upgrade().unwrap().borrow_mut().button_delete().unwrap();


    //                 dialog.close();
    //             });
    //             dialog.show_all();
    //         }else{
    //             // If the previous edit wasnt changed, do normal button behaviour.
    //             listbox.upgrade().unwrap().borrow_mut().button_delete().unwrap();
    //         }



    //     });
    //     button
    // }

    // fn get_button_edit(listbox: Weak<RefCell<CustomListBox>>) -> gtk::Button {
    //     let button: gtk::Button = listbox.upgrade().unwrap().borrow_mut().builder.get_object("button_edit").unwrap();
    //     let window: gtk::Window = listbox.upgrade().unwrap().borrow_mut().builder.get_object("Main_window").unwrap();
    //     button.connect_clicked(move |_| {
    //         if let Some(idx) = Self::confirm_change(Weak::clone(&listbox)).unwrap() {
    //             // If the previous edit had change, do normal button behaviour after new window has closed.
    //             let dialog = Self::get_change_message(&window);
    //             let listbox_ref = Weak::clone(&listbox);

    //             dialog.connect_response(move |dialog, resp| {        
    //                 match resp{
    //                     ResponseType::Ok => {
    //                         listbox_ref.upgrade().unwrap().borrow_mut().values.borrow_mut()[idx].save().unwrap();
    //                     },
    //                     ResponseType::Cancel | ResponseType::DeleteEvent => {
    //                     },
    //                     x => println!("{}", x),
    //                 }
        
    //                 // Normal behaviour here
    //                 listbox_ref.upgrade().unwrap().borrow_mut().button_edit().unwrap();


    //                 dialog.close();
    //             });
    //             dialog.show_all();
    //         }else{
    //             // If the previous edit wasnt changed, do normal button behaviour.
    //             listbox.upgrade().unwrap().borrow_mut().button_edit().unwrap();
    //         }
    //     });
    //     button
    // }

    // fn get_button_new(listbox: Weak<RefCell<CustomListBox>>) -> gtk::Button {
    //     let button: gtk::Button = listbox.upgrade().unwrap().borrow_mut().builder.get_object("button_new").unwrap();
    //     let window: gtk::Window = listbox.upgrade().unwrap().borrow_mut().builder.get_object("Main_window").unwrap();
    //     button.connect_clicked(move |_| {
    //         let confirm = Self::confirm_change(Weak::clone(&listbox)).unwrap();
    //         // A single click is calling Responses more than once. done will make sure that it's done only once for each click.
    //         // let done: bool = false;
    //         let done:Rc<RefCell<DoOnce>> = Rc::new(RefCell::new(DoOnce::new()));
    //         let done_ref = Rc::clone(&done);

    //         // if the last index acessed was modified and not saved
    //         match confirm {
    //             None => {
    //                 listbox.upgrade().unwrap().borrow_mut().button_new().unwrap();
    //             },
    //             Some(idx) => {
    //                 let dialog = Self::get_change_message(&window);
    //                 let listbox_ref = Weak::clone(&listbox);

    //                 dialog.connect_response(move |dialog, resp| {   
    //                     let new_done_ref = Rc::clone(&done_ref);
    //                     match resp{
    //                         ResponseType::Ok => {
    //                             if !new_done_ref.borrow_mut().is_done(){
    //                                 listbox_ref.upgrade().unwrap().borrow_mut().values.borrow_mut()[idx].save().unwrap();
    //                                 listbox_ref.upgrade().unwrap().borrow_mut().button_new().unwrap();
    //                             }
    //                         },
    //                         ResponseType::Cancel | ResponseType::DeleteEvent => {
    //                             if !new_done_ref.borrow_mut().is_done(){
    //                                 listbox_ref.upgrade().unwrap().borrow_mut().button_new().unwrap();
    //                             }
    //                         },
    //                         x => println!("{}", x),
    //                     }
            
    //                     dialog.close();
    //                 });
    //                 dialog.show_all();
    //             }
    //         }

    //     });
    //     button
    // }

    // fn get_button_save(listbox: Weak<RefCell<CustomListBox>>) -> gtk::Button {
    //     let button: gtk::Button = listbox.upgrade().unwrap().borrow_mut().builder.get_object("button_save").unwrap();
    //     button.connect_clicked(move |_| {
    //         listbox.upgrade().unwrap().borrow_mut().button_save().unwrap();
    //     });
    //     button
    // }

    // // fn message_behavior(listbox: Weak<RefCell<CustomListBox>>, button_name: &str, button_behavior: ButtonBehavior) -> gtk::Button{
    // //     let button: gtk::Button = listbox.upgrade().unwrap().borrow_mut().builder.get_object("button_save").unwrap();
    // //     let window: gtk::Window = listbox.upgrade().unwrap().borrow_mut().builder.get_object("Main_window").unwrap();

    // //     button.connect_clicked(move |_| {
    // //         let changed = Self::check_change(Weak::clone(&listbox)).unwrap();

    // //         // This structs asserts that the instruction will be done only once. Each dialog can receive more than 1 response type.
    // //         let done:Rc<RefCell<DoOnce>> = Rc::new(RefCell::new(DoOnce::new()));
    // //         let done_ref = Rc::clone(&done);

    // //         match changed{
                
    // //         }


        
    // //     }

    // // }


    // fn check_change(listbox: Weak<RefCell<CustomListBox>>, window: &gtk::Window) -> Result<Option<gtk::Dialog>, String>{
    //     let last_edited = listbox.upgrade().unwrap().borrow_mut().last_edited;
    //     match last_edited {
    //         LastEdit::NONE => {
    //             return Ok(None);
    //         },
    //         LastEdit::NEW(idx) => {
    //             return Ok(Some(Self::get_cancel_new_message(window)));
    //         },
    //         LastEdit::UNSAVED(idx) => {
    //             return Ok(Some(Self::get_change_message(window)));
    //         }
    //     }

    //     // if last_edited < 0 {
    //     //     return Ok(None);
    //     // }

    //     // let idx: usize = last_edited as usize;
    //     // listbox.upgrade().unwrap().borrow_mut().last_edited = -1;

    //     // if !listbox.upgrade().unwrap().borrow_mut().values.borrow_mut()[idx].entries.is_changed()? {
    //     //     return Ok(None);
    //     // }
    //     // Ok(Some(idx))
    // }


    // fn get_change_message(window: &gtk::Window) -> gtk::Dialog {
    //     let dialog = gtk::Dialog::with_buttons(
    //         Some("Confirm changes?"), 
    //         Some(window), 
    //         gtk::DialogFlags::MODAL, 
    //         &[("Save", ResponseType::Ok), ("Discard", ResponseType::Cancel)],
    //     );

    //     let content_area = dialog.get_content_area();

    //     let label = gtk::Label::new(Some(&"Save or discard changes?"));

    //     content_area.add(&label);
    //     dialog
    // }

    // fn get_cancel_new_message(window: &gtk::Window) -> gtk::Dialog {
    //     let dialog = gtk::Dialog::with_buttons(
    //         Some("Client is missing a few entries. If continued, data will be dropped. Keep editing?"),
    //         Some(window),
    //         gtk::DialogFlags::MODAL,
    //         &[("Continue", ResponseType::Ok), ("Discard", ResponseType::Cancel)],
    //     );

    //     let content_area = dialog.get_content_area();

    //     let label = gtk::Label::new(Some(&"New Client is missing entries"));

    //     content_area.add(&label);
    //     dialog
    // }

    // // fn change_message(idx: usize, listbox: Weak<RefCell<CustomListBox>>, window: &gtk::Window)-> Result<(), String> {
    // //     // println!("Hi");
    // //     let dialog = gtk::Dialog::with_buttons(
    // //         Some("Confirm changes?"), 
    // //         Some(window),
    // //         gtk::DialogFlags::MODAL,
    // //         &[("Save", ResponseType::Ok), ("Discard", ResponseType::Cancel)],
    // //     );

    // //     let content_area = dialog.get_content_area();

    // //     let label = gtk::Label::new(Some(&"Save or discard changes?"));
    // //     // entry.connect_activate(clone!(@weak dialog => move |_| {
    // //     //     dialog.response(ResponseType::Ok);
    // //     // }));
    // //     content_area.add(&label);



    // //     dialog.connect_response(move |dialog, resp| {
    // //         // let text = entry.get_text();
    // //         // if !text.is_empty() && resp == ResponseType::Ok {
    // //         //     model.append(&RowData::new(&text, spin_button.get_value() as u32));
    // //         // }

    // //         match resp{
    // //             ResponseType::Ok => {
    // //                 println!("ResponseType::Ok");
    // //                 listbox.upgrade().unwrap().borrow_mut().values.borrow_mut()[idx].save().unwrap();
    // //                 listbox.upgrade().unwrap().borrow_mut().values.borrow_mut()[idx].clear().unwrap();
    // //             },
    // //             ResponseType::Cancel | ResponseType::DeleteEvent => {
    // //                 println!("ResponseType::Cancel");
    // //                 listbox.upgrade().unwrap().borrow_mut().values.borrow_mut()[idx].clear().unwrap();
    // //             },
    // //             x => println!("{}", x),
    // //         }


    // //         dialog.close();
    // //     });

    // //     dialog.show_all();
    // //     // println!("Window closed");
    // //     Ok(())

    // // }
}
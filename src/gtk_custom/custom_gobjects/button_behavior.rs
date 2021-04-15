use std::rc::{
    Rc,
    Weak,
};
use std::cell::RefCell;

use crate::gtk_custom::custom_gobjects::listbox::CustomListBox;
use crate::gtk_custom::{
    do_once::DoOnce,
    last_edited::LastEdit,
};



use gio::ListStore;
use gtk::prelude::*;
use gtk::Grid;
use gtk::ResponseType;



// This enum holds a type associated with each button. It's behavior change with each of them.
pub enum ButtonBehavior{
    NEW(Weak<RefCell<CustomListBox>>),
    EDIT(Weak<RefCell<CustomListBox>>),
    DELETE(Weak<RefCell<CustomListBox>>),
    SAVE(Weak<RefCell<CustomListBox>>),
    CANCEL(Weak<RefCell<CustomListBox>>),
}

// This stores what each button should do
impl ButtonBehavior{
    pub fn action(&mut self){
        match self{
            ButtonBehavior::NEW(listbox) => {
                println!("ButtonBehavior::NEW");
                listbox.upgrade().unwrap().borrow_mut().button_new().unwrap();
            },
            ButtonBehavior::EDIT(listbox) => {
                println!("ButtonBehavior::EDIT");
                listbox.upgrade().unwrap().borrow_mut().button_edit().unwrap();
            },
            ButtonBehavior::DELETE(listbox) => {
                println!("ButtonBehavior::DELETE");
                listbox.upgrade().unwrap().borrow_mut().button_delete().unwrap();
            },
            ButtonBehavior::SAVE(listbox) => {
                println!("ButtonBehavior::SAVE");
                listbox.upgrade().unwrap().borrow_mut().button_save().unwrap();
            },
            ButtonBehavior::CANCEL(listbox) => {
                println!("ButtonBehavior::CANCEL");
                listbox.upgrade().unwrap().borrow_mut().button_cancel().unwrap();
            }
        }
    }

    pub fn comparison(&self, other: &ButtonBehavior) -> bool {
        match self{
            ButtonBehavior::NEW(_) => {
                if let ButtonBehavior::NEW(_) = other{
                    return true;
                }
                return false;
            },
            ButtonBehavior::EDIT(_) => {
                if let ButtonBehavior::EDIT(_) = other{
                    return true;
                }
                return false;
            },
            ButtonBehavior::DELETE(_) => {
                if let ButtonBehavior::DELETE(_) = other{
                    return true;
                }
                return false;
            },
            ButtonBehavior::SAVE(_) => {
                if let ButtonBehavior::SAVE(_) = other{
                    return true;
                }
                return false;
            },
            ButtonBehavior::CANCEL(_) => {
                if let ButtonBehavior::CANCEL(_) = other{
                    return true;
                }
                return false;
            }

        };
    }

    pub fn get_listbox_ref(&self) -> Weak<RefCell<CustomListBox>> {
        match self{
            ButtonBehavior::NEW(listbox) => {
                return Weak::clone(listbox)
            },
            ButtonBehavior::EDIT(listbox) => {
                return Weak::clone(listbox)
            },
            ButtonBehavior::DELETE(listbox) => {
                return Weak::clone(listbox)
            },
            ButtonBehavior::SAVE(listbox) => {
                return Weak::clone(listbox)
            },
            ButtonBehavior::CANCEL(listbox) => {
                return Weak::clone(listbox)
            }

        };
    }
}

impl Clone for ButtonBehavior{
    fn clone(&self) -> Self {
        match self{
            ButtonBehavior::NEW(listbox) => {
                return ButtonBehavior::NEW(Weak::clone(&listbox));
            },
            ButtonBehavior::EDIT(listbox) => {
                return ButtonBehavior::EDIT(Weak::clone(&listbox));
            },
            ButtonBehavior::DELETE(listbox) => {
                return ButtonBehavior::DELETE(Weak::clone(&listbox));
            },
            ButtonBehavior::SAVE(listbox) => {
                return ButtonBehavior::SAVE(Weak::clone(&listbox));
            },
            ButtonBehavior::CANCEL(listbox) => {
                return ButtonBehavior::CANCEL(Weak::clone(&listbox));
            }
        }
        
    }
}

pub struct CustomButton{
    widget: gtk::Button,
    behavior: Rc<RefCell<ButtonBehavior>>,
    window: gtk::Window,
}

impl CustomButton{


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






    pub fn new(listbox: Weak<RefCell<CustomListBox>>, button_name: &str, button_behavior: ButtonBehavior) -> Self {
        let button: gtk::Button = listbox.upgrade().unwrap().borrow_mut().builder.get_object(button_name).unwrap();
        // window to store in struct
        let window: gtk::Window = listbox.upgrade().unwrap().borrow_mut().builder.get_object("Main_window").unwrap();
        // window to move to closure
        let window_ref: gtk::Window = listbox.upgrade().unwrap().borrow_mut().builder.get_object("Main_window").unwrap();

        // button behavior to store in struct
        let button_behavior = Rc::new(RefCell::new(button_behavior));
        // button behavior to move to closure
        let button_behavior_ref = Rc::downgrade(&button_behavior);
        // button behavior to check pattern
        let pattern = ButtonBehavior::SAVE(button_behavior.borrow().get_listbox_ref());
        // let button_behavior = mut button_behavior;
        
        if button_behavior.borrow().comparison(&pattern) {
            button.connect_clicked(move |_| {
                button_behavior_ref.upgrade().unwrap().borrow_mut().action();
            });
        }else{
            button.connect_clicked(move |_| {
                let changed = Self::check_change(Weak::clone(&listbox), &window_ref).unwrap();
                let button_behavior = Weak::clone(&button_behavior_ref);
                // This structs asserts that the instruction will be done only once. Each dialog can receive more than 1 response type.
                let done:Rc<RefCell<DoOnce>> = Rc::new(RefCell::new(DoOnce::new()));
                // let done_ref = Rc::clone(&done);


                match changed{
                    None => {
                        println!("No change");
                        button_behavior.upgrade().unwrap().borrow_mut().action();
                    },
                    Some((dialog, listbox, last_edited)) => {
                        
                        dialog.connect_response(move |dialog, resp| {   
                            // I'm worried about these references either being dropped during responses, or being a memory leak.
                            let new_done_ref = Rc::clone(&done);
                            // subsequent responses will not be checked
                            if new_done_ref.borrow_mut().is_done(){
                                return
                            }

                            match resp{
                                ResponseType::Ok => {
                                    
                                    match last_edited{
                                        LastEdit::NEW(idx) => {
                                            // Save or Discard changes? yes
                                            println!("Client is missing a few entries. If continued, data will be dropped. Keep editing? Continue.");
                                            listbox.upgrade().unwrap().borrow_mut().edit_from_index(idx).unwrap();
                                        },
                                        LastEdit::UNSAVED(_idx) => {
                                            // Save or Discard changes? yes
                                            println!("Save or Discard changes? yes");
                                            listbox.upgrade().unwrap().borrow_mut().button_save().unwrap();
                                            button_behavior.upgrade().unwrap().borrow_mut().action();
                                        },
                                        LastEdit::NONE => {
                                            unreachable!();
                                        }
                                    }


                                    // if !new_done_ref.borrow_mut().is_done(){
                                    //     listbox.upgrade().unwrap().borrow_mut().values.borrow_mut()[idx].save().unwrap();
                                    //     listbox.upgrade().unwrap().borrow_mut().button_new().unwrap();
                                    // }



                                },
                                ResponseType::Cancel | ResponseType::DeleteEvent => {

                                    match last_edited{
                                        LastEdit::NEW(idx) => {
                                            println!("Client is missing a few entries. If continued, data will be dropped. Keep editing? Discard.");
                                            listbox.upgrade().unwrap().borrow_mut().delete_from_index(idx as i32).unwrap();
                                            button_behavior.upgrade().unwrap().borrow_mut().action();
                                        },
                                        LastEdit::UNSAVED(_) => {
                                            // Save or Discard changes? yes
                                            println!("Save or Discard changes? No.");
                                            listbox.upgrade().unwrap().borrow_mut().button_cancel().unwrap();
                                            button_behavior.upgrade().unwrap().borrow_mut().action();
                                        },
                                        LastEdit::NONE => {
                                            unreachable!();
                                        }
                                    }

                                    // if !new_done_ref.borrow_mut().is_done(){
                                    //     listbox.upgrade().unwrap().borrow_mut().button_new().unwrap();
                                    // }



                                },
                                x => println!("{}", x),
                            }
                
                            dialog.close();
                        });
                        dialog.show_all();

                    }
                }

            
            });



        }

        

        // println!("hi");
        CustomButton{
            widget: button,
            behavior: button_behavior,
            window,
        }
    }

    fn check_change(listbox: Weak<RefCell<CustomListBox>>, window: &gtk::Window) -> Result<Option<(gtk::Dialog, Weak<RefCell<CustomListBox>>, LastEdit)>, String>{
        let last_edited = listbox.upgrade().unwrap().borrow_mut().last_edited.clone();
        match last_edited {
            LastEdit::NONE => {
                println!("No changee");
                return Ok(None);
            },
            LastEdit::NEW(idx) => {
                println!("New window message");
                return Ok(Some((Self::get_cancel_new_message(window), Weak::clone(&listbox), LastEdit::NEW(idx))));
            },
            LastEdit::UNSAVED(idx) => {
                println!("New unsaved message");
                return Ok(Some((Self::get_change_message(window), Weak::clone(&listbox), LastEdit::UNSAVED(idx))));
            }
        }
    }

    fn get_change_message(window: &gtk::Window) -> gtk::Dialog {
        let dialog = gtk::Dialog::with_buttons(
            Some("Confirm changes?"), 
            Some(window), 
            gtk::DialogFlags::MODAL, 
            &[("Save", ResponseType::Ok), ("Discard", ResponseType::Cancel)],
        );

        let content_area = dialog.get_content_area();

        let label = gtk::Label::new(Some(&"Save or discard changes?"));

        content_area.add(&label);
        dialog
    }

    fn get_cancel_new_message(window: &gtk::Window) -> gtk::Dialog {
        let dialog = gtk::Dialog::with_buttons(
            Some("New Client is missing entries"),
            Some(window),
            gtk::DialogFlags::MODAL,
            &[("Continue", ResponseType::Ok), ("Discard", ResponseType::Cancel)],
        );

        let content_area = dialog.get_content_area();

        let label = gtk::Label::new(Some(&"Client is missing a few entries. If continued, data will be dropped. Keep editing?"));

        content_area.add(&label);
        dialog
    }
}
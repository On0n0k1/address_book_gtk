mod listbox;
mod button_new;
mod button_delete;
mod button_edit;
mod button_cancel;
mod button_save;

use gio::ListStore;
use gtk::prelude::*;
use gtk::Grid;

pub use listbox::CustomListBox;
pub use button_new::ButtonNew;
pub use button_delete::ButtonDelete;
pub use button_edit::ButtonEdit;
pub use button_cancel::ButtonCancel;
pub use button_save::ButtonSave;

pub struct CustomGobjects{
    listbox: CustomListBox,
    button_new: ButtonNew,
    button_delete: ButtonDelete,
    button_edit: ButtonEdit,
    button_cancel: ButtonCancel,
    button_save: ButtonSave,
}


impl CustomGobjects{
    pub fn new_and_connect(window: &gtk::Window, builder: &gtk::Builder) -> Self{
        // let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let viewport_for_listbox: gtk::Viewport = builder.get_object("listbox_viewport").unwrap();
        
        let button_cancel: gtk::Button = builder.get_object("button_cancel").unwrap();
        let button_delete: gtk::Button = builder.get_object("button_delete").unwrap();
        let button_edit: gtk::Button = builder.get_object("button_edit").unwrap();
        let button_new: gtk::Button = builder.get_object("button_new").unwrap();
        let button_save: gtk::Button = builder.get_object("button_save").unwrap();
        
        let listbox = CustomListBox::new(window, &viewport_for_listbox);
        
        // viewport_for_listbox.add(&scrolled_window);
        // scrolled_window.add(&listbox.widget);
        viewport_for_listbox.add(&listbox.widget);
        // scrolled_window.add(&viewport_for_listbox);

        let button_cancel: ButtonCancel = ButtonCancel::new_and_connect(
            button_cancel, 
            &listbox.model,
            &listbox.widget,
        );

        let button_delete: ButtonDelete = ButtonDelete::new_and_connect(
            button_delete, 
            &listbox.model,
            &listbox.widget,
        );

        let button_edit: ButtonEdit = ButtonEdit::new_and_connect(
            button_edit, 
            &listbox.model,
            &listbox.widget,
        );

        let button_new: ButtonNew = ButtonNew::new_and_connect(
            button_new, 
            &listbox.model,
            &listbox.widget,
        );

        let button_save: ButtonSave = ButtonSave::new_and_connect(
            button_save, 
            &listbox.model,
            &listbox.widget,
        );
        
        // grid_for_listbox.add(widget)
        // viewport_for_listbox.add(&scrolled_window);
        // viewport_for_listbox.pack_start(&scrolled_window, true, true, 0);

        // scrolled_window.add(&listbox);
        
        CustomGobjects{
            listbox,
            button_new,
            button_delete,
            button_edit,
            button_cancel,
            button_save,
        }
    
    }

}
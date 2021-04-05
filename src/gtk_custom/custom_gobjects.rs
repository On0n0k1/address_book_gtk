mod listbox;
mod button_new;
mod button_delete;
mod button_edit;
mod button_cancel;
mod button_save;

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

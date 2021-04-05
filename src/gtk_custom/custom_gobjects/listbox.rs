use gio::prelude::*;
use gtk::prelude::*;

use gtk::ResponseType;
use glib::clone;

use std::env::args;

use crate::fields::client::RowData;

pub fn hi() -> String{
    String::from("Hi from listbox")
}

pub struct CustomListBox{
    pub widget: gtk::ListBox,
    pub model: gio::ListStore,
}


impl CustomListBox{
    pub fn new(window: &gtk::Window, viewport: &gtk::Viewport) -> Self{

        let model = gio::ListStore::new(RowData::static_type());
        
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

        CustomListBox{
            widget: listbox,
            model,
        }
    
    }

    // search about connect_row_selected

}
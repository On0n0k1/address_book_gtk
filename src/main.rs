#[macro_use]
extern crate glib;
extern crate gtk;
extern crate gio;

mod gtk_custom;
// mod fields;


use gtk::prelude::*;

use gtk_custom::custom_gobjects::CustomGobjects;

pub fn run3(){
    if gtk::init().is_err(){
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("glade/address_book.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("Main_window").unwrap();

    // listbox should be set in position 0 of this grid...
    
    let custom_gobjects = CustomGobjects::new_and_connect(
        &window, 
        builder
    );


    window.connect_delete_event(|_, _| { gtk::main_quit(); Inhibit(false) });
    
    // std::mem::drop(builder);
    window.show_all();

    gtk::main();


}

fn main() {
    run3();
    // println!("{}\n{}\n{}\n{}\n",
    //     // gtk_custom::listbox::hi(),
    //     // fields::client::address::hi(),
    //     // fields::client::email::hi(),
    //     // fields::client::name::hi(),
    //     // fields::client::phone::hi(),
    // );
}
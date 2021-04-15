#[macro_use]
extern crate glib;
extern crate gtk;
extern crate gio;

mod gtk_custom;


use gtk::prelude::*;

use gtk_custom::custom_gobjects::CustomGobjects;

// use std::{
//     cell::RefCell,
//     rc::Rc,
// };


pub fn run3(){
    if gtk::init().is_err(){
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("glade/address_book.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("Main_window").unwrap();
    // let window: Rc<RefCell<gtk::Window>> = Rc::new(RefCell::new(window));


    let custom_gobjects = CustomGobjects::new_and_connect(
        // window, 
        builder
    );


    window.connect_delete_event(|_, _| { gtk::main_quit(); Inhibit(false) });
    
    window.show_all();

    gtk::main();


}

fn main() {
    run3();
}
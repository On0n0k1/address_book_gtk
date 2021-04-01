extern crate gtk;

use gtk::prelude::*;

mod gtk_custom;
mod model;

pub fn run3(){
    if gtk::init().is_err(){
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("glade/address_book.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("Main_window").unwrap();

    window.connect_delete_event(|_, _| { gtk::main_quit(); Inhibit(false) });
    
    std::mem::drop(builder);
    window.show_all();

    gtk::main();


}

fn main() {
    run3();
    println!("{}\n{}\n{}\n{}\n{}\n{}\n",
        gtk_custom::listbox::hi(),
        gtk_custom::liststore::hi(),
        model::client::address::hi(),
        model::client::email::hi(),
        model::client::name::hi(),
        model::client::phone::hi(),
    );
}
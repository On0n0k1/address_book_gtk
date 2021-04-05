#[macro_use]
extern crate glib;
extern crate gtk;
extern crate gio;


use gtk::prelude::*;

mod gtk_custom;
mod fields;

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
    println!("{}\n{}\n{}\n{}\n",
        // gtk_custom::listbox::hi(),
        fields::client::address::hi(),
        fields::client::email::hi(),
        fields::client::name::hi(),
        fields::client::phone::hi(),
    );
}
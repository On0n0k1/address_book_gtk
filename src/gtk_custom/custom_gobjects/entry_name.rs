
use gio::prelude::*;
use gtk::prelude::*;

use glib::clone;



use gtk::Entry;

pub struct EntryName{
    pub first: Entry,
    pub middle: Entry,
    pub last: Entry,
}

impl EntryName{
    pub fn new(builder: &gtk::Builder) -> Self{

    }
}
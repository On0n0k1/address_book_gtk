# address_book_gtk

![address_book_gtk](https://github.com/On0n0k1/address_book_gtk/blob/main/static/address_book_gtk.png)

Address Book implemented in GTK with Rust

Needs to install gtk developer library to test it in ubuntu.

Name of library is libgtk-3-dev. Can be installed with "sudo apt-get install libgtk-3-dev".
As shown in the link https://www.gtk.org/docs/installations/linux/


Docker image doesn't Work yet. Please don't run it. It's taking more than half an hour to install the library there,
didn't have time to test if it work yet.


Steps to run:

 - Install rust by running '''curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh''' Source: https://www.rust-lang.org/tools/install
 
 - (Not sure if required) Install build essentials with '''sudo apt-get install build-essential'''

 - Install libgtk-3-dev by running '''sudo apt-get install libgtk-3-dev'''
 
 - Clone this repository with '''git clone https://github.com/On0n0k1/address_book_gtk.git'''
 
 - cd to this folder and run '''cargo run'''
 
 
Things to consider:

 - Buttons working properly. 

 - When a client is not saved, a window pops up asking to save.

 - When a client is created but not saved, the window will ask to finish editing or delete the unfinished client.

 - If save button is pressed, but there are still invalid entries, a window will prompt the user and not save it (not implemented yet).

 - Each entry will check for it's own validity (Not implemented yet).



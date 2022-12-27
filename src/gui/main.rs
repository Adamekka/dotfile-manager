#[path = "../lib.rs"]
mod lib;

use lib::set_folders;

fn main() {
    println!("dman-gui found on path");
    set_folders();
}

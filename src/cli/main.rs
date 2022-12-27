mod args;
#[path = "../lib.rs"]
mod lib;

use args::match_args;
use lib::set_folders;

fn main() {
    set_folders();
    match_args();
}

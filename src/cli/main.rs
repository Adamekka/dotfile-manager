mod args;
#[path = "../lib.rs"]
mod lib;

use args::match_args;

fn main() {
    match_args();
}

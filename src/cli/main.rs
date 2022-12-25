mod args;
#[path = "../lib.rs"]
mod lib;

use args::arguments;

fn main() {
    match_args();
}

fn match_args() {
    let args = arguments();
    match args.subcommand() {
        Some(("import", set_matches)) => {}
        Some(("pull", set_matches)) => {}
        Some(("pull-all", set_matches)) => {}
        Some(("push", set_matches)) => {}
        Some(("push-all", set_matches)) => {}
        _ => unreachable!(),
    }
}

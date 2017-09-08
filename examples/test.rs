extern crate dyon;

use dyon::{error, run};

fn main() {
    let file = std::env::args_os().nth(1)
        .and_then(|s| s.into_string().ok())
        .unwrap_or(String::from("source/test.dyon"));
    error(run(&file));
}

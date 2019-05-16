extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin
        .lock()
        .lines()
        .next()
        .expect("there was no next line")
        .expect("the line could not be read");
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    ctx.set_contents(line.to_owned()).unwrap();
}

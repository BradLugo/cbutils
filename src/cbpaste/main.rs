extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::io::{self, Write};

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    io::stdout().flush().unwrap();
    match ctx.get_contents() {
        Ok(text) => print!("{:?}", text),
        Err(err) => print!("{:?}", err), // FIXME :: can't print err when expecting clipboard
    }
    io::stdout().flush().unwrap();
}

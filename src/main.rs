extern crate chrono;
extern crate clipboard;

use chrono::Datelike;
use chrono::Local;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::format;

fn main() {
    let now = Local::now();

    let week_num = now.iso_week().week();
    let week_str = format!("{week_num}");

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(week_str.to_owned()).unwrap();

    println!("{week_str}");
}

extern crate leptess;
extern crate stopwatch;

use stopwatch::{Stopwatch};
use leptess::{leptonica, tesseract};
use std::path::Path;
use std::env;

fn main() {
    let mut api = tesseract::TessApi::new(None, "eng").unwrap();

    let args:Vec<String> = env::args().collect();
    let filepath = &args[1];

    let pix = leptonica::pix_read(Path::new(filepath)).unwrap();

    api.set_image(&pix);
    let sw = Stopwatch::start_new();
    let text = api.get_utf8_text();
    println!("{}", text.unwrap());
    println!("Thing took {}ms", sw.elapsed_ms());
}
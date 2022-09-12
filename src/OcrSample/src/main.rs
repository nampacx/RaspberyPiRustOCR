extern crate leptess;
extern crate stopwatch;

use stopwatch::{Stopwatch};
use leptess::{leptonica, tesseract};
use std::path::Path;

fn main() {
    let mut api = tesseract::TessApi::new(None, "eng").unwrap();

    let pix = leptonica::pix_read(Path::new("/mnt/d/temp/di.png")).unwrap();
    api.set_image(&pix);
    let sw = Stopwatch::start_new();
    let text = api.get_utf8_text();
    println!("{}", text.unwrap());
    println!("Thing took {}ms", sw.elapsed_ms());
}
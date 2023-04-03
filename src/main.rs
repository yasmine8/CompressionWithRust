extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
//use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;
use native_dialog::{FileDialog};

fn main() {
    let path = FileDialog::new()
        .set_location("~/Documents")
        .add_filter("PNG Image", &["png"])
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .add_filter("PDF files", &["pdf"])
        .show_open_single_file()
        .unwrap();

        let path = match path {
            Some(path) => path,
            None => return,
        };

 
    // println!("Selected file path: {:?}", path);
    // if args().len() !=3 {
    //     eprintln!("Usage: `source` `target`");
    //     return;
    // }
    let mut input = BufReader::new(File::open(path).unwrap());//args().nth(1).unwrap()
    println!("input {:?}", input);
    let output = File::create("compressed_file").unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    print!(
        "source len:{:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len:{:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}



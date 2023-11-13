use flate2::{
    write::GzEncoder,
    Compression
};

use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(arg) = args.get(1) {
        let mut compressed_data = Vec::new();
        let mut encoder = GzEncoder::new(&mut compressed_data, Compression::default());
        encoder.write_all(arg.as_bytes()).expect("error write all");
        encoder.finish().expect("error finish");
        println!("{} - {:?}", arg, hex::encode_upper(compressed_data));
    }
}

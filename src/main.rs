//! Utility to generate a condensed library file (that can be queried quickly) from the CCCBR's XML
//! format (which contains lots of irrelevant information and is hard to query).

use std::{env::args, time::Instant};

use bellframe::parse_cc_lib;

fn main() {
    // Parse the input XML file
    println!("Parsing XML...");
    let start_time = Instant::now();
    let input_path = args()
        .nth(1)
        .expect("Please specify a file-name for the CCCBR method library's XML file.");
    let xml = std::fs::read_to_string(input_path).expect("Error reading XML file");
    let lib = parse_cc_lib(&xml);
    println!("done in {:?}", Instant::now() - start_time);

    // Deserialize the library, and write it to disk
    println!("\nSerialising to JSON...");
    let start_time = Instant::now();
    let json = lib.to_json().unwrap();
    std::fs::write("cccbr-methods.json", &json).expect("Error writing JSON file");
    println!("done in {:?}", Instant::now() - start_time);
}

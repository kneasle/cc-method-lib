//! Utility to generate a condensed library file (that can be queried quickly) from the CCCBR's XML
//! format (which contains lots of irrelevant information and is hard to query).

use argh::FromArgs;
use std::{path::PathBuf, time::Instant};

use bellframe::parse_cc_lib;

/// Utility to convert the CCCBR's verbose method library into the compact format read by `bellframe`
#[derive(FromArgs, Debug, Clone)]
struct CliArgs {
    /// the location of the Central Council's XML method library
    #[argh(positional)]
    cc_lib_path: PathBuf,

    /// the location of the output file.  Defaults to "cccbr-methods.json"
    #[argh(option, short = 'o', default = "\"cccbr-methods.json\".into()")]
    output_path: PathBuf,
}

fn main() {
    // Parse the CLI args
    let args: CliArgs = argh::from_env();

    // Parse the input XML file
    println!("Reading & parsing XML library...");
    let start_time = Instant::now();
    let xml = std::fs::read_to_string(&args.cc_lib_path).expect("Error reading XML file");
    let lib = parse_cc_lib(&xml);
    println!("done in {:?}", start_time.elapsed());

    // Deserialize the library, and write it to disk
    println!("\nSerialising to JSON...");
    let start_time = Instant::now();
    let json = lib.to_json().unwrap();
    std::fs::write(args.output_path, &json).expect("Error writing JSON file");
    println!("done in {:?}", start_time.elapsed());
}

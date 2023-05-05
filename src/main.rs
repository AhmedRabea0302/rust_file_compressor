extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    // Check if the user didn't provide source and target
    if args().len() != 3 {
        println!("Please Enter: `Source File Name` And `Target File Name`");
        return;
    }

    // Read the source file And Create a new copressed one
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    // Print the source file Length
    println!("Source Length: {:?}", input.get_ref().metadata().unwrap().len());

    // Print the target file Length
    println!("Target Length: {:?}", output.metadata().unwrap().len());

    // Print the compression process execution time
    println!("Compression Process Execution Time: {:?}", start.elapsed());

}
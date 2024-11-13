extern crate csv;

use std::{io, process};

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

// Returns a Boxed error trait, which makes it harder for callers
// to match on specific error types or handle different error conditions
// in a more granular way.
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

extern crate csv;

use std::{env, ffi::OsString, process};

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
    let file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn std::error::Error>> {
    match env::args_os().nth(1) {
        Some(arg1) => Ok(arg1),
        None => Err(From::from("expected 1 argument, but got none")),
    }
}

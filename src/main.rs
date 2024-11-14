extern crate csv;

use std::{env, ffi::OsString, process};

use serde::Deserialize;

type Error = Box<dyn std::error::Error>;

// We don't need to derive `Debug` (which doesn't require Serde), but it's a
// good habit to do it for all your types.
//
// Notice that the field names in this struct are NOT in the same order as
// the fields in the CSV data!
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    latitude: f64,
    longitude: f64,
    population: Option<u64>,
    city: String,
    state: String,
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

// Returns a Boxed error trait, which makes it harder for callers
// to match on specific error types or handle different error conditions
// in a more granular way.
fn run() -> Result<(), Error> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
        // Try this if you don't like each record smushed on one line:
        // println!("{:#?}", record);
    }

    Ok(())
}

fn get_first_arg() -> Result<OsString, Error> {
    match env::args_os().nth(1) {
        Some(arg1) => Ok(arg1),
        None => Err(From::from("expected 1 argument, but got none")),
    }
}

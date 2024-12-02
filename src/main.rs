//Boilerplate file
//
//grab from days mod.rs
pub mod days;
use std::{env, process::ExitCode};

//result returns Ok() or ERROR()
fn entry() -> Result<(), ()> {
    //stuff not mutable in rust
    let mut args = env::args();
    //iterate through args
    args.next().expect("Program path needs to be provided");
    let day = args.next().ok_or_else(|| {
        eprintln!("ERROR: Please provide a day");
        //bubble up the error
    })?;

    //String allocated on heap, &str
    // switch case for day
    match day.as_str() {
        //default case
        "day01" => {
            days::day01::solve();
        }
        _ => {
            eprintln!("ERROR: this day is not implemented");
            return Err(());
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}

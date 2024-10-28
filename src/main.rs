use std::{env, process,};
use rusttasksmanager::{Config, run};

fn main() {
    let conf = Config::build(&mut env::args()).inspect_err(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    }).unwrap();
    if let Err(val) = run(conf) {
        eprintln!("Failed to proccess input: {val}")
    };


    
}


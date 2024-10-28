use std::{env, process,};
use rusttasksmanager::{Config, run};

fn main() {
    let conf = Config::build(&mut env::args()).inspect_err(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    }).unwrap();
    run(conf);


    
}


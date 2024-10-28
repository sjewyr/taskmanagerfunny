use std::io::{self, Write};
use std::{env, fs};
pub fn run(conf: Config){
    let mut f = fs::File::options().append(true).create(true).open(&conf.filename).expect(format!("Unable to open {}", &conf.filename).as_str());
    let mut s = String::new();
    if let Err(err) = io::stdin().read_line(&mut s){
        eprintln!("Error reading input: {}", err);
        return;
    }
    if let Err(err) = f.write(s.as_bytes()){
        eprintln!("Error writing to file: {}", err);
        return;
    }
    println!("{} has been written to {}", s.trim(), &conf.filename);
}
pub struct Config {
    pub filename: String
}
impl Config {
    pub fn build(args: &mut env::Args) -> Result<Config, &'static str>{
        if args.len() < 2{
            return Err("Please provide a filename as an argument");
        }
        args.next();
        let filename = args.next().expect("No filename provided");
        Ok(Config{filename})

    }
}

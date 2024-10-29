use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Seek, Write};
use std::{env, fs, process};

fn decode_input(input: &str) -> Result<Message, &'static str> {
    let input = input.trim().to_lowercase();
    match input.split(' ').nth(0).ok_or("Error parsing args")? {
        "new" => Ok(Message::New(
            input
                .split(' ')
                .nth(1)
                .ok_or("Consider adding a task name after NEW keyword")?
                .to_string(),
        )),
        "del" => Ok(Message::Del(
            input
                .split(' ')
                .nth(1)
                .ok_or("Consider adding a task name after DEL keyword")?
                .to_string(),
        )),
        "exit" => Ok(Message::Exit),
        "list" => Ok(Message::List),
        _ => Err("Invalid command.\nCommand should be one of the following: \n\tnew\n\tdel\n\tlist\n\texit"),
    }
}

fn process_input(input: &str, f: &Config) -> Result<(), Box<dyn Error>> {
    match decode_input(input)? {
        Message::New(file_name) => add_to_file(
            &file_name,
            &mut fs::File::options()
                .create(true)
                .append(true)
                .open(&f.filename)?,
        )?,
        Message::Del(file_name) => delete_from_file(
            &file_name,
            &mut fs::OpenOptions::new()
                .read(true)
                .write(true)
                .append(false)
                .create(true)
                .open(&f.filename)?,
        )?,
        Message::Exit => process::exit(0),
        Message::List => list(&mut fs::File::options().read(true).open(&f.filename)?)?,
    }
    Ok(())
}

fn list(f: &mut File) -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    s.lines()
        .map(|line| line.trim())
        .for_each(|line| println!("{line}"));
    Ok(())
}

fn add_to_file(input: &str, f: &mut File) -> Result<(), Box<dyn Error>> {
    f.write((input.to_owned() + "\n").as_bytes())?;
    Ok(())
}

fn delete_from_file(input: &str, f: &mut File) -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    f.seek(io::SeekFrom::Start(0))?;
    f.set_len(0)?;
    s.lines().filter(|line| !line.eq(&input)).try_for_each(
        |line: &str| -> std::io::Result<()> {
            f.write_all((line.to_owned() + "\n").as_bytes())?;
            Ok(())
        },
    )?;
    Ok(())
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        process_input(&input, &conf)
            .inspect_err(|err| {
                println!("{err}");
            })
            .ok();
    }
}
pub struct Config {
    filename: String,
}
impl Config {
    pub fn build(args: &mut env::Args) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Please provide a filename as an argument");
        }
        args.next();
        let filename = args.next().unwrap();
        Ok(Config { filename })
    }
}

enum Message {
    New(String),
    Del(String),
    List,
    Exit,
}
#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_create_and_write_one_task() {
        fs::remove_file("test1").ok();
        let task_name = "stupid_task";
        let command = "New stupid_task";
        let c = Config {
            filename: "test1".to_owned(),
        };
        process_input(command, &c).unwrap();

        let res = fs::read_to_string("test1").unwrap();
        fs::remove_file("test1").unwrap();
        assert_eq!(task_name, res.trim())
    }
    #[test]
    fn test_create_and_remove() {
        fs::remove_file("test2").ok();
        let c = Config {
            filename: "test2".to_owned(),
        };
        let expected_result = "task1\ntask2\ntask4";
        let commands = vec![
            "new task1",
            "new task2",
            "new task3",
            "new task4",
            "del task3",
        ];

        commands.into_iter().for_each(|command| {
            process_input(command, &c).unwrap();
        });

        let res = fs::read_to_string("test2").unwrap();
        fs::remove_file("test2").unwrap();
        assert_eq!(expected_result, res.trim())
    }
}

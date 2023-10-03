use std::fs;
use std::error::Error;
use colored::*;

pub struct Config{
    pub query : String,
    pub filename : String
}

impl Config {
    pub fn new (args : &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        if query == "cat" {
            if args.len() < 3 {
                return Err("not enough arguments");
            }
            let filename = args[2].clone();
            return Ok(Config { query, filename});
        } else if query == "echo" {
            if args.len() < 3 {
                return Err("not enough arguments");
            }
            let words = &args[2..];
            let sentence = words.join(" ");
            return Ok(Config {query , filename : sentence });
        }else if query == "ls" {
            let dir;
            if args.len() == 2 {
                dir = String::from(".");
            }else {
                dir = args[2].clone();
            }
            println!("{}", dir);
            return Ok(Config { query, filename : dir});
        }
        return Ok(Config {query , filename : String::from("Error!")});
    }
}

pub fn echo(x : &str) {
    println!("{}", x);
}

pub fn cat(config : Config ) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    println!("{}", content );
    Ok(())
}

pub fn ls(config : Config ) -> Result<(), Box<dyn Error>> {
    let files = fs::read_dir(config.filename)?;
    for file in files{
        let file_name = file.unwrap().file_name().into_string();
        let file_name = match file_name {
            Ok(x) => x,
            Err(e) => String::from("Error")
        };
        println!("{}", file_name.green());
    }
    Ok(())
}
use std::env;
use std::process;
use my_lib;

fn main() {
    let args : Vec<String>  = env::args().collect();

    let config = my_lib::Config::new(&args).unwrap_or_else(|err| {
        println!("error parsing arguements : {}", err);
        process::exit(1);
    });

    if config.query == "cat" {
        if let Err(err) = my_lib::cat(config){
            println!("Error! : {}", err);
            process::exit(1);
        }
    }else if config.query == "echo" {
        my_lib::echo(&config.filename);
    }else if config.query == "ls" {
        if let Err(err) = my_lib::ls(config){
            println!("Error! : {}", err);
            process::exit(1);
        }
    }
}
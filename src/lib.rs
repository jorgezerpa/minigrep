use std::fs;
use std::process; 

pub fn run(config:Config) {
    let contents = fs::read_to_string(config.filename).unwrap_or_else(|err| {
        println!("Error reading file: {}",err);
        process::exit(1);
    });
    println!("{}", contents);
}

pub struct Config {
    pub query:String,
    pub filename:String,
}

impl Config {
    pub fn new(args:&[String]) -> Result<Config, &str> {
        if args.len()<3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config { filename, query })
    }
}

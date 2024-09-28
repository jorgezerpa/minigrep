use std::env; // for take command line arguments
use std::fs;
use std::process; 

fn main() {
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("in file {}", config.filename);
    println!("----",);

    run(config);

    
}

fn run(config:Config) {
    let contents = fs::read_to_string(config.filename).unwrap_or_else(|err| {
        println!("Error reading file: {}",err);
        process::exit(1);
    });
    println!("{}", contents);
}

struct Config {
    query:String,
    filename:String,
}

impl Config {
    fn new(args:&[String]) -> Result<Config, &str> {
        if args.len()<3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config { filename, query })
    }
}

use std::env; // for take command line arguments
use std::process; 

use minigrep::Config;

fn main() {
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("in file {}", config.filename);
    println!("----",);

    minigrep::run(config);

    
}


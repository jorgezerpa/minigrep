use std::env; // for take command line arguments
use std::fs;

fn main() {
    let args:Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("in file {}", filename);
    println!("----",);
    
    let contents = fs::read_to_string(filename).expect("Something went wrong reading file.");
    
    println!("{}", contents);
    println!("----",);
}

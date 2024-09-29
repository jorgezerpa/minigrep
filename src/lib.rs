use std::fs;
use std::process; 
use std::env;

pub fn run(config:Config) {
    let contents = fs::read_to_string(config.filename).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}",err);
        process::exit(1);
    });

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }
    else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line); 
    }
}

pub struct Config {
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool, // export MINI_GREP_CASE_SENSITIVE=true || unset MINI_GREP_CASE_SENSITIVE
}

impl Config {
    pub fn new(args:&[String]) -> Result<Config, &str> {
        if args.len()<3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("MINI_GREP_CASE_SENSITIVE").is_err();
    
        Ok(Config { filename, query, case_sensitive })
    }
}

pub fn search<'a>(query:&str, contents:&'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query:&str, contents:&'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape (this line should not be showed on the resulting vector)
";

            assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
thrust me.
";

            assert_eq!(
                vec!["Rust:", "thrust me."], 
                search_case_insensitive(query, contents)
            );
    }

}
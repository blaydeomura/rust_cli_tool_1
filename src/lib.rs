use std::fs; // for file system
use std::error::Error;
use std::env;

// need to make public 
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // loop over each line and print it out
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool, // want search to be case sensitive or not
}



impl Config {
    
    // parse function that takes in command args strings
    // return Config struct
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        
        let query: String = args[1].clone(); //.clone() for no ownership   
        let filename: String = args[2].clone();

        // if case sensitive is not set, we'll get an error
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive }) 
    }
}

// searches txt file
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// searches txt file
pub fn search_case_insensitive<'a>(
    query: &str,         // command line query
    contents: &'a str,   // contents of file
) -> Vec<&'a str> {
    // query is what we read in
    let query = query.to_lowercase();

    // empty results vector
    let mut results = Vec::new();

    // for line in contents of file
    for line in contents.lines() {
        // if lowercase line contains query
        if line.to_lowercase().contains(&query) {
            // add to results
            results.push(line);
        }
    }

    results
}

// creating test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // one result because we shoul donly get one result
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        // we expect this to be query string
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }   

    #[test]
    // one result because we shoul donly get one result
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";

        // we expect this to be query string
        assert_eq!(vec!["Rust:", "Trust me."],
         search_case_insensitive(query, contents));
    } 
}
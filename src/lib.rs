use std::fs; // for file system
use std::error::Error;

// need to make public 
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
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

        Ok(Config { query, filename }) 
    }
}
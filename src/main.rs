use std::env;
use std::fs; // for file system
use std::process;
use std::error::Error;

fn main() {
    // args variable: args gives us an iterator, collect is a collection of strings
    let args: Vec<String> = env::args().collect();
    
    let config: Config = Config::new(&args).unwrap_or_else(|err | {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    
    // parse function that takes in command args strings
    // return Config struct
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        
        let query: String = args[1].clone(); //.clone() for no ownership   
        let filename: String = args[2].clone();

        Ok(Config { query, filename }) 
    }
}


// -----------------------------------------------------------------------
// to run:
// cargo run <word you're looking for> <text file name you're searching>
// ie cargo run frong poem.txt
//
// to set case sensitive off:
// $ export CASE_INSENSITIVE=true
// $ cargo run <word> <txt file name>
//
// to set case sensitive on:
// $ unset CASE_INSENSITIVE
// $ cargo run <word> <txt file name>
//
// -----------------------------------------------------------------------

use std::env;
use std::process;

// import minigreap
use minigrep::Config;
fn main() {
    // args variable: args gives us an iterator, collect is a collection of strings
    let args: Vec<String> = env::args().collect();
    
    //config struct
    let config: Config = Config::new(&args).unwrap_or_else(|err | {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // config struct got passed in into run function
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}


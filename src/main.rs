use std::{env, process};
use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // eprintln!("this iis the vector: {:?}", args);

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Applicstion error: {}", e);

        process::exit(1)
    }
}





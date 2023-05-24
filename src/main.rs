use std::env; // allows us to access the args iterator
use std::process; // allows us to manipulate the running process 

use minigrep::Config; // allows access to the minigrep crates Config struct
                      // - defined in lib.rs

fn main() {
    // construct a Config instance or display an error message and end the
    // program; takes ownership of the env::args() iterator
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}",err);
        process::exit(1);
    });
   
    // execute run, which reads and displays the given files contents
    // or display an error message and end the program
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}",e);
        process::exit(1);
    }
        
}


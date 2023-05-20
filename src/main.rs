use std::env; // allows us to access the args iterator
use std::process; // allows us to manipulate the running process 

use minigrep::Config; // allows access to the minigrep crates Config struct
                      // - defined in lib.rs

fn main() {
    // store arguments in args - those given when 'cargo run' is called
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    
    // construct a Config instance or display an error message and the program
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}",err);
        process::exit(1);
    });
    
    // display a helpful message to the user
    println!("Searching for {}",config.query);
    println!("In file {}",config.file_path);

    // execute run, which reads and displays the given files contents
    // or display an error message and end the program
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}",e);
        process::exit(1);
    }
        
}


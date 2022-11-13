use std::env;
use std::process;

use mini_grep::Config;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        println!("For help type 'mini-grep help'");
        // exit with error code 1
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    print!("In files: ");
    for file in &config.file_list {
        print!(" '{}',", file);
    }
    println!();

    // error handling for the run function
    if let Err(e) = mini_grep::run(config) {
        eprintln!("Application error: {}", e);
        println!("For help type 'mini-grep help'");
        process::exit(1);
    }
}

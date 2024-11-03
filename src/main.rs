use std::{env, process};
use mini_grep::Config;
fn main() {
    let args = env::args().collect::<Vec<_>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) =mini_grep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

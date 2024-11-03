use mini_grep::Config;
use std::{env, process};
fn main() {
    let args = env::args().collect::<Vec<_>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = mini_grep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

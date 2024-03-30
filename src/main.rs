use rustgrep::Config;
use std::env;
use std::process;
fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("searching for {}", config.query);
    println!("In file{}", config.file_path);

    if let Err(e) = rustgrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

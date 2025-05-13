use std::env;

use mingrep::Config;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    // now we need to store these
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // running the search query
    if let Err(e) = mingrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

use colored::Colorize;
use std::{env, process};

use baseic::{run, Opts};

fn main() {
    // Build run options
    let args: Vec<String> = env::args().collect();
    let opts = Opts::build(&args).unwrap_or_else(|err| {
        println!("{}: {}", "error".bright_red(), err);
        process::exit(1);
    });

    // Run base conversion
    run(opts).unwrap_or_else(|err| {
        println!("{}: {}", "error".bright_red(), err);
        process::exit(2);
    })
}

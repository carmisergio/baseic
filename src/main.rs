use std::{env, process};

use baseic::{run, ColorPalette, Opts};

fn main() {
    // Build run options
    let args: Vec<String> = env::args().collect();
    let opts = Opts::build(&args).unwrap_or_else(|err| {
        if err.graceful_exit() {
            // Perform graceful exit
            process::exit(0);
        } else {
            // Print error
            eprintln!("{}: {}", "error".format_error(), err);
            process::exit(1);
        }
    });

    // Run base conversion
    run(opts).unwrap_or_else(|err| {
        eprintln!("{}: {}", "error".format_error(), err);
        process::exit(2);
    })
}

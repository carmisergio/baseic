mod constants;
mod opts;
mod strings;

use std::error::Error;

pub use opts::Opts;

pub fn run(opts: Opts) -> Result<(), Box<dyn Error>> {
    println!("{:#?}", opts);

    Ok(())
}

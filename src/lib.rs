mod constants;
mod opts;

use std::error::Error;

pub use opts::Opts;

pub fn run(opts: Opts) -> Result<(), Box<dyn Error>> {
    // let toml = "";
    // // let toml = "default_outconvs = [\"HEX\", \"bin\"]";

    // let config: Config = toml::from_str(toml).unwrap_or_else(|err| {
    //     println!("Error: {}", err);
    //     process::exit(1);
    // });

    println!("{:#?}", opts);

    Ok(())
}

mod constants;
mod convert;
mod opts;
mod ui;

use convert::{do_convert, ConversionError};
pub use {opts::Opts, ui::ColorPalette};

/// Execute conversion and print results
pub fn run(opts: Opts) -> Result<(), ConversionError> {
    let output = do_convert(opts)?;

    // Print result
    print!("{}", output);

    Ok(())
}

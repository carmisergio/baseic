use std::env;

/// Get usage string
pub fn usage() -> String {
    let bin = env::args().next().unwrap();
    let mut res = String::new();
    res += &format!("Usage: {} [OPTS] [INCONV] INPUT [OUTCONVS]", bin);
    // res += &format!("  example: {} dec 1234 bin hex", bin);
    res
}

/// Get help string
pub fn help() -> String {
    let bin = env::args().next().unwrap();
    let mut res = String::new();
    res += &format!("baseic v{}\n", env!("CARGO_PKG_VERSION"));
    res += &usage();
    res += "\n";
    res += &format!("  OPTS: options (optional)\n");
    res += &format!("    -h: display this message\n");
    res += &format!("  INCONV: input converter (optional)\n");
    res += &format!("    DEC: decimal\n");
    res += &format!("    BIN: binary\n");
    res += &format!("    HEX: hexadecimal\n");
    res += &format!("  INPUT: input value\n");
    res += &format!("  OUTCONVS: output converters (optional)\n");
    res += &format!("    DEC: decimal\n");
    res += &format!("    BIN: binary\n");
    res += &format!("    HEX: hexadecimal\n");
    res += &format!("Example: {} dec 1234 bin hex", bin);
    res
}

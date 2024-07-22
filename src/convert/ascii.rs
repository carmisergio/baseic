use nom::{character::complete::anychar, combinator::map_res, IResult};

use super::{InputConverter, IntermediateValue, OutputConverter};

/// Input converter that accepts ascii characters as inputs
pub struct AsciiInputConverter;

impl InputConverter for AsciiInputConverter {
    fn convert(&self, input: &str) -> Result<IntermediateValue, ()> {
        let (input, val) = parse_ascii_char(input).map_err(|_| ())?;

        // Check if there are unconsumed characters
        if input.len() > 0 {
            return Err(());
        }

        Ok(val)
    }
}

/// Output converter that gives ascii characters as outputs
pub struct AsciiOutputConverter;

impl OutputConverter for AsciiOutputConverter {
    fn convert(&self, input: IntermediateValue) -> Result<String, ()> {
        let input: u8 = input.try_into().map_err(|_| ())?;

        // Don't encode non printable characters
        if input < 32 || input > 126 {
            return Err(());
        }

        return Ok(format!("'{}'", input as char));
    }
}

/// Parse positive hexadecimal integer
fn parse_ascii_char(input: &str) -> IResult<&str, IntermediateValue> {
    map_res(anychar, |c| {
        let val = c as i128;
        // Error on non printable characters
        if val < 32 || val > 126 {
            Err(())
        } else {
            Ok(val)
        }
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_inconv_ok() {
        let tests = [("A", 65), ("0", 48), (" ", 32), ("x", 120), ("~", 126)];
        let conv = AsciiInputConverter;
        for (input, exp) in tests {
            assert_eq!(conv.convert(input).unwrap(), exp);
        }
    }

    #[test]
    fn ascii_inconv_err() {
        let tests = ["", "abcdefg", "\t", "'a'"];
        let conv = AsciiInputConverter;
        for input in tests {
            conv.convert(input).unwrap_err();
        }
    }

    #[test]
    fn ascii_outconv_ok() {
        let tests = [(65, "'A'"), (48, "'0'"), (32, "' '"), (126, "'~'")];
        let conv = AsciiOutputConverter;
        for (input, exp) in tests {
            assert_eq!(conv.convert(input).unwrap(), exp);
        }
    }

    #[test]
    fn ascii_outconv_err() {
        let tests = [-123, 0, 127, 200, 31, 20];
        let conv = AsciiOutputConverter;
        for input in tests {
            conv.convert(input).unwrap_err();
        }
    }
}

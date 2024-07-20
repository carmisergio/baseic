use nom::{
    bytes::complete::tag_no_case,
    combinator::{map_res, opt},
    error::ErrorKind,
    sequence::preceded,
    IResult, InputTakeAtPosition,
};

use super::{InputConverter, IntermediateValue, OutputConverter};

/// Input converter that accepts octal numbers as inputs
pub struct OctInputConverter;

impl InputConverter for OctInputConverter {
    fn convert(&self, input: &str) -> Result<IntermediateValue, ()> {
        let (input, val) = parse_pos_oct_int(input).map_err(|_| ())?;

        // Check if there are unconsumed characters
        if input.len() > 0 {
            return Err(());
        }

        Ok(val)
    }
}

/// Output converter that gives octal numbers as outputs
pub struct OctOutputConverter;

impl OutputConverter for OctOutputConverter {
    fn convert(&self, input: IntermediateValue) -> Result<String, ()> {
        // Don't encode negative values
        if input < 0 {
            return Err(());
        }
        return Ok(format!("{:o}", input));
    }
}

/// Parse positive binary integer
fn parse_pos_oct_int(input: &str) -> IResult<&str, IntermediateValue> {
    map_res(preceded(opt(tag_no_case("0o")), oct_digit1), |digits| {
        IntermediateValue::from_str_radix(digits, 8)
    })(input)
}

/// Parses at least one binary digit
fn oct_digit1(input: &str) -> IResult<&str, &str> {
    input.split_at_position1_complete(|item| !(item >= '0' && item <= '7'), ErrorKind::Digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oct_inconv_ok() {
        let tests = [
            ("162635", 0o162635),
            ("0o1234", 0o1234),
            ("0O1234", 0o1234),
            ("00000", 0),
            ("000001", 1),
            ("0o0", 0),
            ("0", 0),
        ];
        let conv = OctInputConverter;
        for (input, exp) in tests {
            assert_eq!(conv.convert(input).unwrap(), exp);
        }
    }

    #[test]
    fn oct_inconv_err() {
        let tests = ["", "abcdefg", "8765", "0o", "0b10010"];
        let conv = OctInputConverter;
        for input in tests {
            conv.convert(input).unwrap_err();
        }
    }

    #[test]
    fn oct_outconv_ok() {
        let tests = [(0o1754, "1754"), (0, "0")];
        let conv = OctOutputConverter;
        for (input, exp) in tests {
            assert_eq!(conv.convert(input).unwrap(), exp);
        }
    }

    #[test]
    fn oct_outconv_err() {
        let tests = [-123];
        let conv = OctOutputConverter;
        for input in tests {
            conv.convert(input).unwrap_err();
        }
    }
}

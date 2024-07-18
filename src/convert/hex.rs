use nom::{
    bytes::complete::tag_no_case,
    character::complete::hex_digit1,
    combinator::{map_res, opt},
    sequence::preceded,
    IResult,
};

// TODO: add -h postfix. Ex: 23h

use super::{InputConverter, IntermediateValue, OutputConverter};

/// Input converter that accepts hexadecimal numbers as inputs
pub struct HexInputConverter;

impl InputConverter for HexInputConverter {
    fn convert(&self, input: &str) -> Result<IntermediateValue, ()> {
        let (input, val) = parse_pos_hex_int(input).map_err(|_| ())?;

        // Check if there are unconsumed characters
        if input.len() > 0 {
            return Err(());
        }

        Ok(val)
    }
}

/// Output converter that gives hexadecimal numbers as outputs
pub struct HexOutputConverter;

impl OutputConverter for HexOutputConverter {
    fn convert(&self, input: IntermediateValue) -> Result<String, ()> {
        return Ok(format!("{:X}", input));
    }
}

/// Parse positive hexadecimal integer
fn parse_pos_hex_int(input: &str) -> IResult<&str, IntermediateValue> {
    map_res(preceded(opt(tag_no_case("0x")), hex_digit1), |digits| {
        IntermediateValue::from_str_radix(digits, 16)
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_inconv_ok() {
        let tests = [
            ("1234ABC", 0x1234ABC),
            ("1234abc", 0x1234ABC),
            ("0xFF", 0xFF),
            ("0XFF", 0xFF),
            ("00000", 0),
            ("0", 0),
            ("0x0", 0),
        ];
        let conv = HexInputConverter;
        for (input, exp) in tests {
            assert_eq!(conv.convert(input).unwrap(), exp);
        }
    }

    #[test]
    fn hex_inconv_err() {
        let tests = ["", "abcdefg", "+*&#", "0x", "aBC0x1234"];
        let conv = HexInputConverter;
        for input in tests {
            conv.convert(input).unwrap_err();
        }
    }

    #[test]
    fn hex_outconv_ok() {
        let tests = [(0x1234ABCD, "1234ABCD"), (0, "0")];
        let conv = HexOutputConverter;
        for (input, exp) in tests {
            assert_eq!(conv.convert(input).unwrap(), exp);
        }
    }
}

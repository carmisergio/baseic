use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::preceded,
    IResult,
};

use super::{InputConverter, IntermediateValue, OutputConverter};

/// Input converter that accepts decimal numbers as inputs
pub struct DecInputConverter;

impl InputConverter for DecInputConverter {
    fn convert(&self, input: &str) -> Result<IntermediateValue, ()> {
        let (input, val) = parse_dec_int(input).map_err(|_| ())?;

        // Check if there are unconsumed characters
        if input.len() > 0 {
            return Err(());
        }

        Ok(val)
    }
}

/// Output converter that gives decimal numbers as outputs
pub struct DecOutputConverter;

impl OutputConverter for DecOutputConverter {
    fn convert(&self, input: IntermediateValue) -> Result<String, ()> {
        return Ok(format!("{}", input));
    }
}

/// Parse decimal integer
fn parse_dec_int(input: &str) -> IResult<&str, IntermediateValue> {
    alt((parse_neg_dec_int, parse_pos_dec_int))(input)
}

/// Parse negative decimal integer
fn parse_neg_dec_int(input: &str) -> IResult<&str, IntermediateValue> {
    map(preceded(tag("-"), parse_pos_dec_int), |val| -val)(input)
}

/// Parse positive decimal integer
fn parse_pos_dec_int(input: &str) -> IResult<&str, IntermediateValue> {
    map_res(digit1, |digits| {
        IntermediateValue::from_str_radix(digits, 10)
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dec_inconv_ok() {
        let tests = [("1234", 1234), ("0", 0), ("-145", -145), ("-0", 0)];
        let conv = DecInputConverter;
        for (input, exp) in tests {
            assert_eq!(conv.convert(input).unwrap(), exp);
        }
    }

    #[test]
    fn dec_inconv_err() {
        let tests = ["", "abcde", "123abc", "-+ciao", "-"];
        let conv = DecInputConverter;
        for input in tests {
            conv.convert(input).unwrap_err();
        }
    }

    #[test]
    fn dec_outconv_ok() {
        let tests = [(1234, "1234"), (0, "0"), (-145, "-145")];
        let conv = DecOutputConverter;
        for (input, exp) in tests {
            assert_eq!(conv.convert(input).unwrap(), exp);
        }
    }
}

use nom::{
    bytes::complete::tag_no_case,
    combinator::{map_res, opt},
    error::ErrorKind,
    sequence::preceded,
    IResult, InputTakeAtPosition,
};

use super::{InputConverter, IntermediateValue, OutputConverter};

/// Input converter that accepts hexadecimal numbers as inputs
pub struct BinInputConverter;

impl InputConverter for BinInputConverter {
    fn convert(&self, input: &str) -> Result<IntermediateValue, ()> {
        let (input, val) = parse_pos_bin_int(input).map_err(|_| ())?;

        // Check if there are unconsumed characters
        if input.len() > 0 {
            return Err(());
        }

        Ok(val)
    }
}

/// Output converter that gives hexadecimal numbers as outputs
pub struct BinOuptutConverter;

impl OutputConverter for BinOuptutConverter {
    fn convert(&self, input: IntermediateValue) -> Result<String, ()> {
        return Ok(format!("{:b}", input));
    }
}

/// Parse positive binary integer
fn parse_pos_bin_int(input: &str) -> IResult<&str, IntermediateValue> {
    map_res(preceded(opt(tag_no_case("0b")), bin_digit1), |digits| {
        dbg!(&digits);
        IntermediateValue::from_str_radix(digits, 2)
    })(input)
}

/// Parses at least one binary digit
fn bin_digit1(input: &str) -> IResult<&str, &str> {
    input.split_at_position1_complete(|item| !(item == '0' || item == '1'), ErrorKind::Digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_inconv_ok() {
        let tests = [
            ("10110011010", 0b10110011010),
            ("0b1000", 0b1000),
            ("0B1000", 0b1000),
            ("00000", 0),
            ("000001", 1),
            ("0b0", 0),
            ("0", 0),
        ];
        let conv = BinInputConverter;
        for (input, exp) in tests {
            assert_eq!(conv.convert(input).unwrap(), exp);
        }
    }

    #[test]
    fn bin_inconv_err() {
        let tests = ["", "abcdefg", "1234", "0b", "0x10010"];
        let conv = BinInputConverter;
        for input in tests {
            conv.convert(input).unwrap_err();
        }
    }

    #[test]
    fn bin_outconv_ok() {
        let tests = [(0b101101, "101101"), (0, "0")];
        let conv = BinOuptutConverter;
        for (input, exp) in tests {
            assert_eq!(conv.convert(input).unwrap(), exp);
        }
    }
}

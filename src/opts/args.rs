use std::{fmt::Display, str::FromStr};

use nom::{
    combinator::{map, opt},
    error::{Error, ErrorKind, ParseError},
    multi::many0,
    Err, IResult,
};

use super::conv_types::{InputConverterType, OutputConverterType};
use crate::ui::help;

/// Representation of the cli arguments
#[derive(Debug, PartialEq)]
pub struct ArgVals {
    pub input: String,
    pub inconv: Option<InputConverterType>,
    pub outconvs: Option<Vec<OutputConverterType>>,
    pub opts: CliOptions,
}

impl ArgVals {
    /// Parse args
    /// Args format: [OPTS] [INCONV] INPUT [OUTCONVS]
    pub fn from_args(args: &[String]) -> Result<ArgVals, ArgParseError> {
        parse_arguments(&args[1..])
            .map(|(_, vals)| vals)
            .map_err(|e| match e {
                Err::Incomplete(_) => panic!(),
                Err::Failure(e) | Err::Error(e) => e,
            })
    }
}

/// Arugment parsing error
#[derive(Debug, PartialEq)]
pub enum ArgParseError<'a> {
    MissingInput,
    UnknownOutputConverter(&'a str),
    InvalidCliOption(&'a str),
    Eof,
    GracefulExit,
}

/// Representation of the command line options set
#[derive(Debug, PartialEq)]
pub struct CliOptions {
    pub help: bool,
}

impl CliOptions {
    pub fn new() -> Self {
        Self { help: false }
    }
}

#[derive(Debug, PartialEq)]
enum CliOptionToken {
    Help,
}

impl FromStr for CliOptionToken {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-h" => Ok(Self::Help),
            _ => Err(()),
        }
    }
}

impl<'a> Display for ArgParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingInput => write!(f, "input value is required"),
            Self::UnknownOutputConverter(conv) => {
                write!(f, "unknown ouptut converter: \"{}\"", conv)
            }
            Self::InvalidCliOption(opt) => {
                write!(f, "invalid cli option: \"{}\"", opt)
            }
            Self::Eof => {
                write!(f, "missing required argument")
            }
            Self::GracefulExit => {
                /* Should never be displayed */
                Ok(())
            }
        }
    }
}

/// Parse arguments
fn parse_arguments(input: &[String]) -> IResult<&[String], ArgVals, ArgParseError> {
    // Parse options
    let (input, opts) = parse_cli_options(input).expect("should never fail");

    // Check if -h option set
    if opts.help {
        // Print help
        eprintln!("{}", help());
        return Err(Err::Error(ArgParseError::GracefulExit));
    }

    // Parse arguments
    let (input, inconv) =
        opt(parse_fromstr::<InputConverterType>)(input).map_err(|_| panic!("optional"))?;
    let (input, inval) = any(input).map_err(|_| Err::Error(ArgParseError::MissingInput))?;
    let (input, outconvs) = parse_outconvs_list(input)?;

    Ok((
        input,
        ArgVals {
            input: inval.clone(),
            inconv,
            outconvs,
            opts,
        },
    ))
}

/// Parse CLI options
fn parse_cli_options(input: &[String]) -> IResult<&[String], CliOptions> {
    let (input, opt_tokens) = many0(parse_cli_opt)(input)?;

    // Extract settings from options
    let mut opts = CliOptions::new();
    for opt in opt_tokens {
        match opt {
            CliOptionToken::Help => opts.help = true,
        }
    }

    Ok((input, opts))
}

/// Parse CLI Option
fn parse_cli_opt(input: &[String]) -> IResult<&[String], CliOptionToken> {
    parse_cli_opt_help(input)
}

fn parse_cli_opt_help(input: &[String]) -> IResult<&[String], CliOptionToken> {
    map(tag_token("-h"), |_| CliOptionToken::Help)(input)
}

/// Parse output converter type
fn parse_outconv_type(input: &[String]) -> IResult<&[String], OutputConverterType, ArgParseError> {
    let (input, s) = any(input).map_err(|_| Err::Error(ArgParseError::Eof))?;
    let (_, val) = parse_fromstr(&[s.clone()])
        .map_err(|_| Err::Error(ArgParseError::UnknownOutputConverter(s)))?;
    Ok((input, val))
}

/// Parse output converters list
fn parse_outconvs_list(
    mut input: &[String],
) -> IResult<&[String], Option<Vec<OutputConverterType>>, ArgParseError> {
    let mut outconvs = Vec::new();

    while input.len() > 0 {
        // Parse token
        match parse_outconv_type(input) {
            Ok((rem, val)) => {
                // Add value to output vector
                outconvs.push(val);

                // Update next token
                input = rem
            }
            Err(e) => return Err(e),
        }
    }

    // Map output converters to None if none were entered
    let outconvs = if outconvs.len() > 0 {
        Some(outconvs)
    } else {
        None
    };

    Ok((input, outconvs))
}

/// Parse first token using FromStr
fn parse_fromstr<T: FromStr>(input: &[String]) -> IResult<&[String], T> {
    // Check that there is a token to parse
    if input.len() < 1 {
        return Err(Err::Error(ParseError::from_error_kind(
            input,
            ErrorKind::Eof,
        )));
    }

    // Parse token
    let val = match input[0].parse::<T>() {
        Ok(val) => val,
        Err(_) => {
            return Err(Err::Error(ParseError::from_error_kind(
                input,
                ErrorKind::IsNot,
            )));
        }
    };

    // Return result and remaining tokens
    Ok((&input[1..], val))
}

/// Checks if first token matches input string
fn tag_token<'a>(tag: &'a str) -> impl Fn(&[String]) -> IResult<&[String], &String> + 'a {
    move |input| {
        let (rem, val) = any(input)?;
        if val == tag {
            Ok((rem, val))
        } else {
            Err(Err::Error(Error::from_error_kind(input, ErrorKind::Tag)))
        }
    }
}

/// Consume any token as string
fn any(input: &[String]) -> IResult<&[String], &String> {
    // Check that there is a token to parse
    if input.len() < 1 {
        return Err(Err::Error(Error::from_error_kind(input, ErrorKind::Eof)));
    }

    Ok((&input[1..], &input[0]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fromstr_inconv_ok() {
        let tests = [
            (
                vec!["HEX".to_string(), "ciaone".to_string(), "".to_string()],
                vec!["ciaone".to_string(), "".to_string()],
                InputConverterType::HEX,
            ),
            (vec!["bin".to_string()], vec![], InputConverterType::BIN),
            (
                vec!["DEC".to_string(), "".to_string()],
                vec!["".to_string()],
                InputConverterType::DEC,
            ),
        ];

        for (input, exprem, exp) in tests {
            let (rem, out) = parse_fromstr::<InputConverterType>(&input).unwrap();
            assert_eq!(rem, exprem);
            assert_eq!(out, exp);
        }
    }

    #[test]
    fn parse_fromstr_inconv_err() {
        let tests = [vec![], vec!["hello".to_string(), "".to_string()]];

        for input in tests {
            parse_fromstr::<InputConverterType>(&input).unwrap_err();
        }
    }

    #[test]
    fn parse_any_ok() {
        let tests = [
            (
                vec![
                    "test1".to_string(),
                    "test2".to_string(),
                    "test3".to_string(),
                ],
                vec!["test2".to_string(), "test3".to_string()],
                "test1".to_string(),
            ),
            (vec!["hello".to_string()], vec![], "hello".to_string()),
        ];

        for (input, exprem, exp) in tests {
            let (rem, out) = any(&input).unwrap();
            assert_eq!(rem, exprem);
            assert_eq!(*out, exp);
        }
    }

    #[test]
    fn parse_any_err() {
        let tests = [vec![]];

        for input in tests {
            any(&input).unwrap_err();
        }
    }

    #[test]
    fn tag_token_ok() {
        let tests = [
            (
                vec!["hello".to_string(), "test".to_string()],
                "hello",
                vec!["test".to_string()],
                "hello".to_string(),
            ),
            (
                vec!["dennis".to_string()],
                "dennis",
                vec![],
                "dennis".to_string(),
            ),
        ];

        for (input, tag, exprem, exp) in tests {
            let (rem, out) = tag_token(tag)(&input).unwrap();
            assert_eq!(rem, exprem);
            assert_eq!(*out, exp);
        }
    }

    #[test]
    fn tag_token_err() {
        let tests = [(vec![], "test"), (vec!["token".to_string()], "test")];

        for (input, tag) in tests {
            tag_token(tag)(&input).unwrap_err();
        }
    }

    #[test]
    fn parse_cli_options_ok() {
        let tests = [
            (vec![], vec![], CliOptions { help: false }),
            (
                vec!["test".to_string()],
                vec!["test".to_string()],
                CliOptions { help: false },
            ),
            (
                vec!["-h".to_string(), "test".to_string()],
                vec!["test".to_string()],
                CliOptions { help: true },
            ),
        ];

        for (input, exprem, exp) in tests {
            let (rem, out) = parse_cli_options(&input).unwrap();
            assert_eq!(rem, exprem);
            assert_eq!(out, exp);
        }
    }

    #[test]
    fn parse_arguments_ok() {
        let tests = [
            (
                vec!["test1".to_string()],
                ArgVals {
                    input: "test1".to_string(),
                    inconv: None,
                    outconvs: None,
                    opts: CliOptions { help: false },
                },
            ),
            (
                vec!["hex".to_string(), "test2".to_string()],
                ArgVals {
                    input: "test2".to_string(),
                    inconv: Some(InputConverterType::HEX),
                    outconvs: None,
                    opts: CliOptions { help: false },
                },
            ),
            (
                vec!["test3".to_string(), "hex".to_string(), "dec".to_string()],
                ArgVals {
                    input: "test3".to_string(),
                    inconv: None,
                    outconvs: Some(vec![OutputConverterType::HEX, OutputConverterType::DEC]),
                    opts: CliOptions { help: false },
                },
            ),
            (
                vec![
                    "bin".to_string(),
                    "test4".to_string(),
                    "hex".to_string(),
                    "bin".to_string(),
                ],
                ArgVals {
                    input: "test4".to_string(),
                    inconv: Some(InputConverterType::BIN),
                    outconvs: Some(vec![OutputConverterType::HEX, OutputConverterType::BIN]),
                    opts: CliOptions { help: false },
                },
            ),
        ];

        for (input, exp) in tests {
            let (_, out) = parse_arguments(&input).unwrap();
            assert_eq!(out, exp);
        }
    }

    #[test]
    fn parse_arguments_err() {
        let tests = [
            (vec![], ArgParseError::MissingInput),
            (
                vec!["test".to_string(), "test2".to_string()],
                ArgParseError::UnknownOutputConverter("test2"),
            ),
            (vec!["-h".to_string()], ArgParseError::GracefulExit),
        ];

        for (input, experr) in tests {
            let err = parse_arguments(&input).unwrap_err();
            match err {
                Err::Failure(_) | Err::Incomplete(_) => panic!(),
                Err::Error(e) => assert_eq!(e, experr),
            }
        }
    }
}

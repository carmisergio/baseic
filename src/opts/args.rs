use std::{fmt::Display, str::FromStr};

use indexmap::IndexSet;
use nom::{
    combinator::opt,
    error::{Error, ErrorKind, ParseError},
    multi::many0,
    Err, IResult,
};

use crate::{
    convert::{InputConverterType, OutputConverterType},
    ui::{help, version},
};

/// Representation of the cli arguments
#[derive(Debug, PartialEq)]
pub struct ArgVals {
    pub input: String,
    pub inconv: Option<InputConverterType>,
    pub outconvs: Option<IndexSet<OutputConverterType>>,
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
    pub version: bool,
}

impl CliOptions {
    pub fn new() -> Self {
        Self {
            help: false,
            version: false,
        }
    }
}

#[derive(Debug, PartialEq)]
enum CliOptionToken {
    Help,
    Version,
}

impl FromStr for CliOptionToken {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-h" => Ok(Self::Help),
            "-v" => Ok(Self::Version),
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

    // Act upon options
    handle_options(opts)?;

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
        },
    ))
}

fn handle_options<'a>(opts: CliOptions) -> IResult<(), (), ArgParseError<'a>> {
    // Check if -h option set
    if opts.help {
        // Print help
        eprintln!("{}", help());
        return Err(Err::Error(ArgParseError::GracefulExit));
    }
    // Check if -v option set
    if opts.version {
        // Print help
        eprintln!("{}", version());
        return Err(Err::Error(ArgParseError::GracefulExit));
    }
    Ok(((), ()))
}

/// Parse CLI options
fn parse_cli_options(input: &[String]) -> IResult<&[String], CliOptions> {
    let (input, opt_tokens) = many0(parse_cli_opt)(input)?;

    // Extract settings from options
    let mut opts = CliOptions::new();
    for opt in opt_tokens {
        match opt {
            CliOptionToken::Help => opts.help = true,
            CliOptionToken::Version => opts.version = true,
        }
    }

    Ok((input, opts))
}

/// Parse CLI Option
fn parse_cli_opt(input: &[String]) -> IResult<&[String], CliOptionToken> {
    parse_cli_opt_help(input)
}

fn parse_cli_opt_help(input: &[String]) -> IResult<&[String], CliOptionToken> {
    // map(tag_token("-h"), |_| CliOptionToken::Help)(input)
    parse_fromstr(input)
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
) -> IResult<&[String], Option<IndexSet<OutputConverterType>>, ArgParseError> {
    let mut outconvs = IndexSet::new();

    while input.len() > 0 {
        // Parse token
        match parse_outconv_type(input) {
            Ok((rem, val)) => {
                // Add value to output vector
                outconvs.insert(val);

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
    use indexmap::indexset;

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
    fn parse_cli_options_ok() {
        let tests = [
            (
                vec![],
                vec![],
                CliOptions {
                    help: false,
                    version: false,
                },
            ),
            (
                vec!["test".to_string()],
                vec!["test".to_string()],
                CliOptions {
                    help: false,
                    version: false,
                },
            ),
            (
                vec!["-h".to_string(), "test".to_string()],
                vec!["test".to_string()],
                CliOptions {
                    help: true,
                    version: false,
                },
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
                },
            ),
            (
                vec!["hex".to_string(), "test2".to_string()],
                ArgVals {
                    input: "test2".to_string(),
                    inconv: Some(InputConverterType::HEX),
                    outconvs: None,
                },
            ),
            (
                vec!["test3".to_string(), "hex".to_string(), "dec".to_string()],
                ArgVals {
                    input: "test3".to_string(),
                    inconv: None,
                    outconvs: Some(indexset! {OutputConverterType::HEX, OutputConverterType::DEC}),
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
                    outconvs: Some(indexset! {OutputConverterType::HEX, OutputConverterType::BIN}),
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
            (vec!["-v".to_string()], ArgParseError::GracefulExit),
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

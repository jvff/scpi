use std::str;
use std::str::FromStr;

use nom::{digit, ErrorKind, IResult, rest};

use super::request::ScpiRequest;

pub trait ScpiParameterParser {
    fn parse(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
}

fn make_true<T>(_: T) -> bool {
    true
}

fn make_false<T>(_: T) -> bool {
    false
}

impl ScpiParameterParser for bool {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        named!(parser(&[u8]) -> bool,
            alt!(
                map!(tag!("true"), make_true) |
                map!(tag!("false"), make_false)
            )
        );

        parser(input)
    }
}

impl ScpiParameterParser for usize {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        named!(parser(&[u8]) -> usize,
            map_res!(
                map_res!(
                    recognize!(many1!(digit)),
                    str::from_utf8
                ),
                usize::from_str
            )
        );

        parser(input)
    }
}

impl ScpiParameterParser for String {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        named!(parser(&[u8]) -> String,
            map!(map_res!(rest, str::from_utf8), String::from)
        );

        parser(input)
    }
}

impl<T> ScpiParameterParser for T
where
    T: ScpiRequest,
{
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        named!(parser(&[u8]) -> &str,
            map_res!(rest, str::from_utf8)
        );

        match parser(input) {
            IResult::Done(input, string) => {
                match Self::decode(string) {
                    Some(instance) => IResult::Done(input, instance),
                    None => IResult::Error(ErrorKind::MapRes),
                }
            }
            IResult::Error(error) => IResult::Error(error),
            IResult::Incomplete(needed) => IResult::Incomplete(needed),
        }
    }
}

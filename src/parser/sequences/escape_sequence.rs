use crate::traits::parser::Parser;
use crate::errors::json_parser_error::JsonParserError;
use crate::errors::json_parser_error::JsonParserError::{InvalidEscapeSequenceOpeningToken, InvalidEscapeSequenceToken, UnexpectedEndOfData};
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

const BACKTICK: char = '\\';

pub struct ParserEscapeSequence {}

impl Parser for ParserEscapeSequence {
    fn parse(stream: &mut JsonStream) -> Result<Property, JsonParserError> {
        stream.skip_whitespaces();

        match stream.peek() {
            Some(BACKTICK) => (),
            Some(token) => return Err(InvalidEscapeSequenceOpeningToken(token)),
            None => return Err(UnexpectedEndOfData),
        }

        stream.consume(1).unwrap();

        let replaced_character = match stream.next() {
            Some(BACKTICK) => BACKTICK,
            Some('/') => '/',
            Some('"') => '"',
            Some('b') => '\x08',
            Some('f') => '\x0c',
            Some('n') => '\n',
            Some('r') => '\r',
            Some('t') => '\t',
            Some(token) => return Err(InvalidEscapeSequenceToken(token)),
            None => return Err(UnexpectedEndOfData)
        };

        return Ok(Property {
            number: None,
            string: Some(String::from(replaced_character)),
            array: None,
            object: None,
            bool: None,
            is_null: false,
        });
    }
}
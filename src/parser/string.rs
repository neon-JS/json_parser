use crate::traits::parser::Parser;
use crate::errors::json_parser_error::JsonParserError;
use crate::errors::json_parser_error::JsonParserError::{InvalidStringOpeningToken, UnexpectedEndOfData};
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

const BACKTICK: char = '\\';
pub const DOUBLE_QUOTES: char = '"';

pub struct ParserString {}

// TODO: Special & escaped characters, line breaks etc.
impl Parser for ParserString {
    fn parse(stream: &mut JsonStream) -> Result<Property, JsonParserError> {
        stream.skip_whitespaces();

        match stream.peek() {
            Some(DOUBLE_QUOTES) => (),
            Some(token) => return Err(InvalidStringOpeningToken(token)),
            None => return Err(UnexpectedEndOfData),
        }

        stream.consume(1).unwrap();

        let mut char_is_escaped = false;
        let mut characters = Vec::new();

        while let Some(character) = stream.next() {
            if character == DOUBLE_QUOTES && !char_is_escaped {
                return Ok(Property {
                    number: None,
                    string: Some(String::from(&characters.into_iter().collect::<String>())),
                    array: None,
                    object: None,
                    bool: None,
                    is_null: false,
                });
            }

            if character == BACKTICK && !char_is_escaped {
                // Escape next character
                char_is_escaped = true;
                continue;
            } else if !char_is_escaped {
                // TODO: Handle escaped chars!
                characters.push(character);
            }

            if char_is_escaped {
                char_is_escaped = false
            }
        }

        return Err(UnexpectedEndOfData);
    }
}
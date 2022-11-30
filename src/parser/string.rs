use crate::constants::token::{
    ESC_SEQUENCE_CHARACTER, ESC_UNICODE_IDENTIFIER, STRING_END, STRING_START,
};
use crate::errors::json_parser_error::JsonParserError::{
    InvalidStringOpeningToken, UnexpectedEndOfData,
};
use crate::traits::parser::Parser;
use crate::errors::json_parser_error::JsonParserError;
use crate::parser::sequences::escape_sequence::ParserEscapeSequence;
use crate::parser::sequences::unicode_sequence::ParserUnicodeSequence;
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

pub struct ParserString {}

impl Parser for ParserString {
    fn parse(stream: &mut JsonStream) -> Result<Property, JsonParserError> {
        stream.skip_whitespaces();

        match stream.peek() {
            Some(STRING_START) => (),
            Some(token) => return Err(InvalidStringOpeningToken(token)),
            None => return Err(UnexpectedEndOfData),
        }

        stream.consume(1).unwrap();

        let mut result = String::new();

        while let Some(character) = stream.peek() {
            if character == STRING_END {
                stream.consume(1).unwrap();

                return Ok(Property {
                    number: None,
                    string: Some(result),
                    array: None,
                    object: None,
                    bool: None,
                    is_null: false,
                });
            }

            if character == ESC_SEQUENCE_CHARACTER {
                let sequence_property = if stream.peek_equals(ESC_UNICODE_IDENTIFIER) {
                    ParserUnicodeSequence::parse(stream)
                } else {
                    ParserEscapeSequence::parse(stream)
                };

                match sequence_property {
                    Ok(escaped_char) => {
                        result.push_str(escaped_char.string.unwrap().as_str());
                        continue;
                    }
                    Err(inner) => return Err(inner)
                }
            }

            result.push(character);
            stream.consume(1).unwrap();
        }

        Err(UnexpectedEndOfData)
    }
}
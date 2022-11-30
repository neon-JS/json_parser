use crate::constants::token::{
    ESC_BACKSPACE, ESC_CARRIAGE_RETURN, ESC_FORM_FEED, ESC_LINE_FEED, ESC_QUOTATION_MARK,
    ESC_REVERSE_SOLIDUS, ESC_SEQUENCE_CHARACTER, ESC_SOLIDUS, ESC_TABULATION,
};
use crate::errors::json_parser_error::JsonParserError::{
    InvalidEscapeSequenceOpeningToken, InvalidEscapeSequenceToken, UnexpectedEndOfData,
};
use crate::traits::parser::Parser;
use crate::errors::json_parser_error::JsonParserError;
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

pub struct ParserEscapeSequence {}

impl Parser for ParserEscapeSequence {
    fn parse(stream: &mut JsonStream) -> Result<Property, JsonParserError> {
        stream.skip_whitespaces();

        match stream.peek() {
            Some(ESC_SEQUENCE_CHARACTER) => (),
            Some(token) => return Err(InvalidEscapeSequenceOpeningToken(token)),
            None => return Err(UnexpectedEndOfData),
        }

        stream.consume(1).unwrap();

        let replaced_character = match stream.next() {
            Some(ESC_CARRIAGE_RETURN) => '\r',
            Some(ESC_REVERSE_SOLIDUS) => '\\',
            Some(ESC_QUOTATION_MARK) => '"',
            Some(ESC_TABULATION) => '\t',
            Some(ESC_BACKSPACE) => '\x08',
            Some(ESC_FORM_FEED) => '\x0c',
            Some(ESC_LINE_FEED) => '\n',
            Some(ESC_SOLIDUS) => '/',
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
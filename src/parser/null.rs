use crate::constants::token::TOKEN_NULL;
use crate::errors::json_parser_error::JsonParserError::{InvalidNullToken};
use crate::traits::parser::Parser;
use crate::errors::json_parser_error::JsonParserError;
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

pub struct ParserNull {}

impl Parser for ParserNull {
    fn parse(stream: &mut JsonStream) -> Result<Property, JsonParserError> {
        stream.skip_whitespaces();

        if stream.peek_equals(TOKEN_NULL) {
            stream.consume(4).unwrap();
            return Ok(Property {
                number: None,
                string: None,
                array: None,
                object: None,
                bool: None,
                is_null: true,
            });
        }

        Err(InvalidNullToken)
    }
}
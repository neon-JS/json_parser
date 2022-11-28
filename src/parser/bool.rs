use crate::traits::parser::Parser;
use crate::errors::json_parser_error::JsonParserError;
use crate::errors::json_parser_error::JsonParserError::{InvalidBoolToken};
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

pub struct ParserBool {}

impl Parser for ParserBool {
    fn parse(stream: &mut JsonStream) -> Result<Property, JsonParserError> {
        stream.skip_whitespaces();

        if stream.peek_equals("true") {
            stream.consume(4).unwrap();
            return Ok(Property {
                number: None,
                string: None,
                array: None,
                object: None,
                bool: Some(true),
                is_null: false,
            });
        }

        if stream.peek_equals("false") {
            stream.consume(5).unwrap();
            return Ok(Property {
                number: None,
                string: None,
                array: None,
                object: None,
                bool: Some(false),
                is_null: false,
            });
        }

        return Err(InvalidBoolToken);
    }
}
use crate::traits::parser::Parser;
use crate::errors::json_parser_error::JsonParserError;
use crate::errors::json_parser_error::JsonParserError::{InvalidArrayOpeningToken, InvalidArrayProperty, UnexpectedEndOfData};
use crate::parser::root::ParserRoot;
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

pub const ARRAY_OPENING_BRACKET: char = '[';
const ARRAY_CLOSING_BRACKET: char = ']';
const PROPERTY_SEPARATOR: char = ',';

pub struct ParserArray {}

impl Parser for ParserArray {
    fn parse(stream: &mut JsonStream) -> Result<Property, JsonParserError> {
        stream.skip_whitespaces();

        match stream.peek() {
            Some(ARRAY_OPENING_BRACKET) => (),
            Some(token) => return Err(InvalidArrayOpeningToken(token)),
            None => return Err(UnexpectedEndOfData),
        }

        stream.consume(1).unwrap();

        let mut properties = Vec::new();

        loop {
            stream.skip_whitespaces();

            match stream.peek() {
                Some(ARRAY_CLOSING_BRACKET) => {
                    stream.consume(1).unwrap();
                    return Ok(Property {
                        number: None,
                        string: None,
                        array: Some(properties),
                        object: None,
                        bool: None,
                        is_null: false,
                    });
                }
                Some(PROPERTY_SEPARATOR) => {
                    stream.consume(1).unwrap();
                }
                None => return Err(UnexpectedEndOfData),
                _ => ()
            }

            let property = match ParserRoot::parse(stream) {
                Ok(p) => p,
                Err(inner) => return Err(InvalidArrayProperty(Box::new(inner))),
            };

            properties.push(property);
        }
    }
}
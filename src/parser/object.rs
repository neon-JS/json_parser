use std::collections::HashMap;
use crate::traits::parser::Parser;
use crate::errors::json_parser_error::JsonParserError;
use crate::errors::json_parser_error::JsonParserError::{
    DuplicateKey, InvalidObjectKey, InvalidObjectKeyValueSeparatorToken, InvalidObjectOpeningToken,
    UnexpectedEndOfData, InvalidObjectProperty,
};
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;
use crate::parser::root::ParserRoot;
use crate::parser::string::ParserString;

pub const OBJECT_OPENING_BRACKET: char = '{';
const OBJECT_CLOSING_BRACKET: char = '}';
const PROPERTY_SEPARATOR: char = ',';
const KEY_VALUE_SEPARATOR: char = ':';

pub struct ParserObject {}

impl Parser for ParserObject {
    fn parse(stream: &mut JsonStream) -> Result<Property, JsonParserError> {
        stream.skip_whitespaces();

        match stream.peek() {
            Some(OBJECT_OPENING_BRACKET) => (),
            Some(token) => return Err(InvalidObjectOpeningToken(token)),
            None => return Err(UnexpectedEndOfData),
        }

        stream.consume(1).unwrap();

        let mut properties: HashMap<String, Box<Property>> = HashMap::new();

        loop {
            stream.skip_whitespaces();

            match stream.peek() {
                Some(OBJECT_CLOSING_BRACKET) => {
                    stream.consume(1).unwrap();
                    return Ok(Property {
                        number: None,
                        string: None,
                        array: None,
                        object: Some(properties),
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

            let key = match ParserString::parse(stream) {
                Ok(property) => property.string.unwrap(),
                Err(inner) => return Err(InvalidObjectKey(Box::new(inner))),
            };

            stream.skip_whitespaces();
            match stream.next() {
                Some(KEY_VALUE_SEPARATOR) => (),
                Some(token) => return Err(InvalidObjectKeyValueSeparatorToken(token)),
                None => return Err(UnexpectedEndOfData),
            }

            let property = match ParserRoot::parse(stream) {
                Ok(p) => p,
                Err(inner) => return Err(InvalidObjectProperty(Box::new(inner))),
            };

            if properties.contains_key(key.as_str()) {
                return Err(DuplicateKey(key));
            }

            properties.insert(key, Box::new(property));
        }
    }
}
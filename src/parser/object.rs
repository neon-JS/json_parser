use crate::constants::token::{
    OBJECT_BRACKET_CLOSE, OBJECT_BRACKET_OPEN, OBJECT_KEY_VALUE_SEPARATOR, OBJECT_PROPERTY_SEPARATOR,
};
use crate::errors::json_parser_error::JsonParserError::{
    DuplicateKey, InvalidObjectKey, InvalidObjectKeyValueSeparatorToken, InvalidObjectOpeningToken,
    UnexpectedEndOfData, InvalidObjectProperty, InvalidObjectPrecedingSeparator
};
use std::collections::HashMap;
use crate::traits::parser::Parser;
use crate::errors::json_parser_error::JsonParserError;
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;
use crate::parser::root::ParserRoot;
use crate::parser::string::ParserString;

pub struct ParserObject {}

impl Parser for ParserObject {
    fn parse(stream: &mut JsonStream) -> Result<Property, JsonParserError> {
        stream.skip_whitespaces();

        match stream.next() {
            Some(OBJECT_BRACKET_OPEN) => (),
            Some(token) => return Err(InvalidObjectOpeningToken(token)),
            None => return Err(UnexpectedEndOfData),
        }

        let mut properties: HashMap<String, Box<Property>> = HashMap::new();

        loop {
            stream.skip_whitespaces();

            match stream.peek() {
                Some(OBJECT_BRACKET_CLOSE) => {
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
                Some(OBJECT_PROPERTY_SEPARATOR) => {
                    if properties.is_empty() {
                        return Err(InvalidObjectPrecedingSeparator)
                    }
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
                Some(OBJECT_KEY_VALUE_SEPARATOR) => (),
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
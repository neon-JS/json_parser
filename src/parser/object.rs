use std::collections::HashMap;
use crate::definitions::parser::Parser;
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
    fn parse(stream: &mut JsonStream) -> Result<Property, String> {
        stream.skip_whitespaces();

        match stream.peek() {
            Some(OBJECT_OPENING_BRACKET) => (),
            None => return Err(String::from("Could not parse object: Too short.")),
            _ => return Err(String::from("Could not parse object: Missing opening braces."))
        }

        stream.consume(1).unwrap();

        let mut properties: HashMap<String, Box<Property>> = HashMap::new();

        loop {
            stream.skip_whitespaces();

            match stream.peek() {
                Some(OBJECT_CLOSING_BRACKET) => {
                    stream.consume(1).unwrap();
                    return Ok(Property {
                        numeric_value: None,
                        string_value: None,
                        array_value: None,
                        object_value: Some(properties),
                        bool_value: None,
                        is_null_value: false,
                    });
                }
                Some(PROPERTY_SEPARATOR) => {
                    stream.consume(1).unwrap();
                }
                None => return Err(String::from("Could not parse object: Missing closing braces.")),
                _ => ()
            }

            let key = match ParserString::parse(stream) {
                Ok(property) => property.string_value.unwrap(),
                Err(_) => return Err(String::from("Could not parse object: Invalid key")),
            };

            stream.skip_whitespaces();
            if stream.next() != Some(KEY_VALUE_SEPARATOR) {
                return Err(String::from("Could not parse object: Missing separator."));
            }

            let property = match ParserRoot::parse(stream) {
                Ok(p) => p,
                Err(e) => return Err(String::from(format!("Could not parse object as property could not be parsed: {}", e))),
            };

            if properties.contains_key(key.as_str()) {
                return Err(String::from("Could not parse object: Duplicate key"));
            }

            properties.insert(key, Box::new(property));
        }
    }
}
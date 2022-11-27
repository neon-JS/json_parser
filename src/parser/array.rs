use crate::definitions::parser::Parser;
use crate::parser::root::ParserRoot;
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

pub const ARRAY_OPENING_BRACKET: char = '[';
const ARRAY_CLOSING_BRACKET: char = ']';
const PROPERTY_SEPARATOR: char = ',';

pub struct ParserArray {}

impl Parser for ParserArray {
    fn parse(stream: &mut JsonStream) -> Result<Property, String> {
        stream.skip_whitespaces();

        match stream.peek() {
            Some(ARRAY_OPENING_BRACKET) => (),
            None => return Err(String::from("Could not parse array: Too short.")),
            _ => return Err(String::from("Could not parse string: Missing opening bracket."))
        }

        stream.consume(1).unwrap();

        let mut properties = Vec::new();

        loop {
            stream.skip_whitespaces();

            match stream.peek() {
                Some(ARRAY_CLOSING_BRACKET) => {
                    stream.consume(1).unwrap();
                    return Ok(Property {
                        numeric_value: None,
                        string_value: None,
                        array_value: Some(properties),
                        object_value: None,
                        bool_value: None,
                        is_null_value: false,
                    });
                },
                Some(PROPERTY_SEPARATOR) => {
                    stream.consume(1).unwrap();
                }
                None => return Err(String::from("Could not parse array: Missing closing bracket.")),
                _ => ()
            }

            let property = match ParserRoot::parse(stream) {
                Ok(p) => p,
                Err(e) => return Err(String::from(format!("Could not parse array as property could not be parsed: {}", e))),
            };

            properties.push(property);
        }
    }
}
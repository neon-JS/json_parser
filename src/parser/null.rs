use crate::definitions::parser::Parser;
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

pub struct ParserNull {}

impl Parser for ParserNull {
    fn parse(stream: &mut JsonStream) -> Result<Property, String> {
        stream.skip_whitespaces();

        if stream.peek_equals("null") {
            stream.consume(4).unwrap();
            return Ok(Property {
                numeric_value: None,
                string_value: None,
                array_value: None,
                object_value: None,
                bool_value: None,
                is_null_value: true,
            });
        }

        return Err(String::from("Could not parse null: Missing token."));
    }
}
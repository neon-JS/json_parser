use crate::definitions::parser::Parser;
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

pub struct ParserBool {}

impl Parser for ParserBool {
    fn parse(stream: &mut JsonStream) -> Result<Property, String> {
        stream.skip_whitespaces();

        if stream.peek_equals("true") {
            stream.consume(4).unwrap();
            return Ok(Property {
                numeric_value: None,
                string_value: None,
                array_value: None,
                object_value: None,
                bool_value: Some(true),
                is_null_value: false,
            });
        }

        if stream.peek_equals("false") {
            stream.consume(5).unwrap();
            return Ok(Property {
                numeric_value: None,
                string_value: None,
                array_value: None,
                object_value: None,
                bool_value: Some(false),
                is_null_value: false,
            });
        }

        return Err(String::from("Could not parse bool: Missing token."));
    }
}
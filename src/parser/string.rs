use crate::definitions::parser::Parser;
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

const BACKTICK: char = '\\';
pub const DOUBLE_QUOTES: char = '"';

pub struct ParserString {}

// TODO: Special & escaped characters, line breaks etc.
impl Parser for ParserString {
    fn parse(stream: &mut JsonStream) -> Result<Property, String> {
        stream.skip_whitespaces();

        match stream.peek() {
            Some(DOUBLE_QUOTES) => (),
            None => return Err(String::from("Could not parse string: Too short.")),
            _ => return Err(String::from("Could not parse string: Missing opening quotes."))
        }

        stream.consume(1).unwrap();

        let mut char_is_escaped = false;
        let mut characters = Vec::new();

        while let Some(character) = stream.next() {
            if character == DOUBLE_QUOTES && !char_is_escaped {
                return Ok(Property {
                    numeric_value: None,
                    string_value: Some(String::from(&characters.into_iter().collect::<String>())),
                    array_value: None,
                    object_value: None,
                    bool_value: None,
                    is_null_value: false,
                });
            }

            if character == BACKTICK && !char_is_escaped {
                // Escape next character
                char_is_escaped = true;
                continue;
            } else if !char_is_escaped {
                // TODO: Handle escaped chars!
                characters.push(character);
            }

            if char_is_escaped {
                char_is_escaped = false
            }
        }

        return Err(String::from("Could not parse string: Missing closing quotes."));
    }
}
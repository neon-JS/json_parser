use crate::parser::Parser;
use crate::property::{Property, ToProperty};

pub struct ContainerString {
    pub value: String
}

// TODO: Special characters, line breaks etc.
impl Parser for ContainerString {
    fn parse<'a>(&mut self, stream: &'a str) -> Result<(Property, &'a str), String> {
        if stream.len() < 2 {
            return Err(String::from("Could not parse string: Too short."));
        }

        if stream.chars().nth(0) != Some('"') {
            return Err(String::from("Could not parse string: Missing token."));
        }

        let mut char_is_escaped = false;

        for (index, char) in stream.chars().enumerate().skip(1) {
            if char == '\\' && !char_is_escaped {
                // Escape next character
                char_is_escaped = true;
                continue;
            }

            if char == '"' && !char_is_escaped {
                // End of string
                self.value = String::from(&stream[1..index]);

                return Ok((self.convert_to_property(), &stream[(index + 1)..]));
            }

            if char_is_escaped {
                char_is_escaped = false
            }
        }

        return Err(String::from("Could not parse string: Missing token."));
    }
}

impl ToProperty for ContainerString {
    fn convert_to_property(&self) -> Property {
        Property {
            numeric_value: None,
            string_value: Some(String::from(&self.value)),
            array_value: None,
            object_value: None,
            bool_value: None,
            is_null_value: false,
        }
    }
}
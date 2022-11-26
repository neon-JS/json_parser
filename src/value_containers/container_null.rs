use crate::definitions::parser::Parser;
use crate::definitions::property::{Property, ToProperty};

pub struct ContainerNull { }

impl Parser for ContainerNull {
    fn parse<'a>(&mut self, stream: &'a str) -> Result<(Property, &'a str), String> {
        if stream.len() >= 4 && stream[..4].eq("null") {
            return Ok((self.convert_to_property(), &stream[4..]));
        }

        return Err(String::from("Could not parse null: Missing token."));
    }
}

impl ToProperty for ContainerNull {
    fn convert_to_property(&self) -> Property {
        Property {
            numeric_value: None,
            string_value: None,
            array_value: None,
            object_value: None,
            bool_value: None,
            is_null_value: true,
        }
    }
}
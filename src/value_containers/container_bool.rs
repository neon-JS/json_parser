use crate::parser::Parser;
use crate::property::{Property, ToProperty};

pub struct ContainerBool {
    pub value: bool
}

impl Parser for ContainerBool {
    fn parse<'a>(&mut self, stream: &'a str) -> Result<(Property, &'a str), String> {
        if stream.len() >= 4 && stream[..4].eq("true") {
            self.value = true;
            return Ok((self.convert_to_property(), &stream[4..]));
        }

        if stream.len() >= 5 && stream[..5].eq("false") {
            self.value = false;
            return Ok((self.convert_to_property(), &stream[5..]));
        }

        return Err(String::from("Could not parse bool: Missing token."));
    }
}

impl ToProperty for ContainerBool {
    fn convert_to_property(&self) -> Property {
        Property {
            numeric_value: None,
            string_value: None,
            array_value: None,
            object_value: None,
            bool_value: Some(self.value),
            is_null_value: false,
        }
    }
}
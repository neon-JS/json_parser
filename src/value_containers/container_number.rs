use crate::definitions::parser::Parser;
use crate::definitions::property::{Property, ToProperty};

pub struct ContainerNumber {
    pub value: f64
}

impl Parser for ContainerNumber {
    fn parse<'a>(&mut self, stream: &'a str) -> Result<(Property, &'a str), String> {
        if stream.len() < 1
        {
            return Err(String::from("Could not parse number: Too short."));
        }

        if !['-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&stream.chars().nth(0).unwrap()) {
            return Err(String::from("Could not parse number: Missing token."));
        }

        let mut contains_separator = false;
        let mut contains_exponent = false;

        let mut end_index= 0;

        for (index, char) in stream.chars().enumerate().skip(1) {
            end_index = index;

            if char >= '0' && char <= '9' {
                continue;
            }

            if char == '.' {
                if !contains_separator {
                    contains_separator = true;
                    continue;
                } else {
                    return Err(String::from("Could not parse number: Incorrectly formed."))
                }
            }

            if char == 'e' || char == 'E' {
                if !contains_exponent {
                    contains_exponent = true;
                    continue;
                } else {
                    return Err(String::from("Could not parse number: Incorrectly formed."))
                }
            }

            break;
        }

        if ['e', 'E'].contains(&stream.chars().nth(end_index).unwrap()) {
            return Err(String::from("Could not parse number: Missing exponent."));
        }

        let parsed_value = stream[..end_index].parse::<f64>();
        if parsed_value.is_err() {
            return Err(String::from("Could not parse number: Invalid format"));
        }

        self.value = parsed_value.unwrap();
        return Ok((self.convert_to_property(), &stream[end_index..]));
    }
}

impl ToProperty for ContainerNumber {
    fn convert_to_property(&self) -> Property {
        Property {
            numeric_value: Some(self.value),
            string_value: None,
            array_value: None,
            object_value: None,
            bool_value: None,
            is_null_value: false,
        }
    }
}
use crate::definitions::parser::Parser;
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

pub const MINUS: char = '-';
const DECIMAL_POINT: char = '.';
const LOWER_EXPONENT_SIGN: char = 'e';
const UPPER_EXPONENT_SIGN: char = 'E';

pub struct ParserNumber {}

impl Parser for ParserNumber {
    fn parse(stream: &mut JsonStream) -> Result<Property, String> {
        stream.skip_whitespaces();

        match stream.peek() {
            Some(MINUS)
            | Some('0')
            | Some('1')
            | Some('2')
            | Some('3')
            | Some('4')
            | Some('5')
            | Some('6')
            | Some('7')
            | Some('8')
            | Some('9')
            => (),
            None => return Err(String::from("Could not parse number: Too short.")),
            _ => return Err(String::from("Could not parse number: Missing token."))
        }

        let mut contains_decimal_point = false;
        let mut contains_exponent = false;
        let mut characters = Vec::new();

        characters.push(stream.next().unwrap());

        while let Some(character) = stream.peek() {
            if character == DECIMAL_POINT {
                if contains_decimal_point {
                    return Err(String::from("Could not parse number: Contains multiple decimal points."));
                }
                contains_decimal_point = true;
            }

            if character == LOWER_EXPONENT_SIGN || character == UPPER_EXPONENT_SIGN {
                if contains_exponent {
                    return Err(String::from("Could not parse number: Contains multiple exponents."));
                }
                contains_exponent = true;
            }

            if
                (character >= '0' && character <= '9')
                || character == LOWER_EXPONENT_SIGN
                || character == UPPER_EXPONENT_SIGN
                || character == DECIMAL_POINT
            {
                stream.consume(1).unwrap();
                characters.push(character);
                continue;
            }

            break;
        }

        if [LOWER_EXPONENT_SIGN, UPPER_EXPONENT_SIGN, MINUS].contains(characters.last().unwrap()) {
            return Err(String::from("Could not parse number: Missing values."));
        }

        let parsed_value = characters.into_iter().collect::<String>().parse::<f64>();
        if parsed_value.is_err() {
            return Err(String::from("Could not parse number: Invalid format"));
        }

        return Ok(Property {
            numeric_value: Some(parsed_value.unwrap()),
            string_value: None,
            array_value: None,
            object_value: None,
            bool_value: None,
            is_null_value: false,
        });
    }
}
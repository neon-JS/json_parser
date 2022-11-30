use crate::constants::token::{
    NUM_EIGHT, NUM_FIVE, NUM_FOUR, NUM_NINE, NUM_ONE, NUM_SEVEN, NUM_SIX, NUM_THREE, NUM_TWO,
    NUM_ZERO, NUM_MINUS, NUM_DECIMAL_POINT, NUM_EXPONENT_SIGN_LOWERCASE,
    NUM_EXPONENT_SIGN_UPPERCASE,
};
use crate::errors::json_parser_error::JsonParserError::{
    InvalidNumberFormat, InvalidNumberToken, MultipleNumberDecimalPoints,
    MultipleNumberExponentSigns, UnexpectedEndOfData,
};
use crate::traits::parser::Parser;
use crate::errors::json_parser_error::JsonParserError;
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

pub struct ParserNumber {}

impl Parser for ParserNumber {
    fn parse(stream: &mut JsonStream) -> Result<Property, JsonParserError> {
        stream.skip_whitespaces();

        match stream.peek() {
            Some(NUM_MINUS)
            | Some(NUM_ZERO)
            | Some(NUM_ONE)
            | Some(NUM_TWO)
            | Some(NUM_THREE)
            | Some(NUM_FOUR)
            | Some(NUM_FIVE)
            | Some(NUM_SIX)
            | Some(NUM_SEVEN)
            | Some(NUM_EIGHT)
            | Some(NUM_NINE)
            => (),
            Some(token) => return Err(InvalidNumberToken(token)),
            None => return Err(UnexpectedEndOfData),
        }

        let mut contains_decimal_point = false;
        let mut contains_exponent = false;
        let mut characters = vec!(stream.next().unwrap());

        while let Some(character) = stream.peek() {
            if character == NUM_DECIMAL_POINT {
                if contains_decimal_point {
                    return Err(MultipleNumberDecimalPoints);
                }
                contains_decimal_point = true;
            }

            if character == NUM_EXPONENT_SIGN_LOWERCASE || character == NUM_EXPONENT_SIGN_UPPERCASE {
                if contains_exponent {
                    return Err(MultipleNumberExponentSigns);
                }
                contains_exponent = true;
            }

            if (NUM_ZERO..=NUM_NINE).contains(&character)
                || character == NUM_EXPONENT_SIGN_LOWERCASE
                || character == NUM_EXPONENT_SIGN_UPPERCASE
                || character == NUM_DECIMAL_POINT
            {
                stream.consume(1).unwrap();
                characters.push(character);
                continue;
            }

            break;
        }

        match characters.last() {
            Some(&NUM_EXPONENT_SIGN_LOWERCASE)
            | Some(&NUM_EXPONENT_SIGN_UPPERCASE)
            | Some(&NUM_MINUS)
            => return Err(UnexpectedEndOfData),
            _ => ()
        };

        let number_as_string = characters.into_iter().collect::<String>();

        match number_as_string.parse::<f64>() {
            Ok(number) => Ok(Property {
                number: Some(number),
                string: None,
                array: None,
                object: None,
                bool: None,
                is_null: false,
            }),
            Err(_) => Err(InvalidNumberFormat(number_as_string))
        }
    }
}
use crate::constants::token::{
    NUM_EIGHT, NUM_FIVE, NUM_FOUR, NUM_NINE, NUM_ONE, NUM_SEVEN, NUM_SIX, NUM_THREE, NUM_TWO,
    NUM_ZERO, NUM_MINUS, NUM_PLUS, NUM_DECIMAL_POINT, NUM_EXPONENT_SIGN_LOWERCASE,
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
            Some(
                NUM_MINUS
                | NUM_ZERO
                | NUM_ONE
                | NUM_TWO
                | NUM_THREE
                | NUM_FOUR
                | NUM_FIVE
                | NUM_SIX
                | NUM_SEVEN
                | NUM_EIGHT
                | NUM_NINE
            )
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

            if character == NUM_MINUS || character == NUM_PLUS {
                match characters.last() {
                    Some(&(NUM_EXPONENT_SIGN_LOWERCASE | NUM_EXPONENT_SIGN_UPPERCASE)) => (),
                    Some(_) => return Err(InvalidNumberToken(character)),
                    None => return Err(UnexpectedEndOfData),
                }
            }

            match character {
                NUM_ZERO..=NUM_NINE
                | NUM_EXPONENT_SIGN_LOWERCASE
                | NUM_EXPONENT_SIGN_UPPERCASE
                | NUM_DECIMAL_POINT
                | NUM_MINUS
                | NUM_PLUS
                => {
                    stream.consume(1).unwrap();
                    characters.push(character);
                    continue;
                }
                _ => break
            }
        }

        if let Some(&(NUM_EXPONENT_SIGN_LOWERCASE | NUM_EXPONENT_SIGN_UPPERCASE | NUM_MINUS | NUM_PLUS)) = characters.last() {
            return Err(UnexpectedEndOfData);
        }

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
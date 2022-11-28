use crate::traits::parser::Parser;
use crate::errors::json_parser_error::JsonParserError;
use crate::errors::json_parser_error::JsonParserError::{InvalidNumberFormat, InvalidNumberToken, MultipleNumberDecimalPoints, MultipleNumberExponentSigns, UnexpectedEndOfData};
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

pub const MINUS: char = '-';
const DECIMAL_POINT: char = '.';
const LOWER_EXPONENT_SIGN: char = 'e';
const UPPER_EXPONENT_SIGN: char = 'E';

pub struct ParserNumber {}

impl Parser for ParserNumber {
    fn parse(stream: &mut JsonStream) -> Result<Property, JsonParserError> {
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
            Some(token) => return Err(InvalidNumberToken(token)),
            None => return Err(UnexpectedEndOfData),
        }

        let mut contains_decimal_point = false;
        let mut contains_exponent = false;
        let mut characters = Vec::new();

        characters.push(stream.next().unwrap());

        while let Some(character) = stream.peek() {
            if character == DECIMAL_POINT {
                if contains_decimal_point {
                    return Err(MultipleNumberDecimalPoints);
                }
                contains_decimal_point = true;
            }

            if character == LOWER_EXPONENT_SIGN || character == UPPER_EXPONENT_SIGN {
                if contains_exponent {
                    return Err(MultipleNumberExponentSigns);
                }
                contains_exponent = true;
            }

            if (character >= '0' && character <= '9')
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

        match characters.last() {
            Some(&LOWER_EXPONENT_SIGN)
            | Some(&UPPER_EXPONENT_SIGN)
            | Some(&MINUS)
            => return Err(UnexpectedEndOfData),
            _ => ()
        };

        let number_as_string = characters.into_iter().collect::<String>();

        return match number_as_string.parse::<f64>() {
            Ok(number) => Ok(Property {
                number: Some(number),
                string: None,
                array: None,
                object: None,
                bool: None,
                is_null: false,
            }),
            Err(_) => Err(InvalidNumberFormat(number_as_string))
        };
    }
}
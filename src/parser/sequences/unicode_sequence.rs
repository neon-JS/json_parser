use crate::traits::parser::Parser;
use crate::errors::json_parser_error::JsonParserError;
use crate::errors::json_parser_error::JsonParserError::{InvalidEscapeSequence, InvalidEscapeSequenceOpeningToken, InvalidEscapeSequenceToken, UnexpectedEndOfData};
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

const BACKTICK: char = '\\';

pub struct ParserUnicodeSequence {}

impl Parser for ParserUnicodeSequence {
    fn parse(stream: &mut JsonStream) -> Result<Property, JsonParserError> {
        stream.skip_whitespaces();

        match stream.peek() {
            Some(BACKTICK) => (),
            Some(token) => return Err(InvalidEscapeSequenceOpeningToken(token)),
            None => return Err(UnexpectedEndOfData),
        }
        stream.consume(1).unwrap();

        match stream.next() {
            Some('u') => (),
            Some(token) => return Err(InvalidEscapeSequenceToken(token)),
            None => return Err(UnexpectedEndOfData)
        };
        stream.consume(1).unwrap();

        let mut unicode_value;

        match ParserUnicodeSequence::parse_utf_16_code_unit(stream) {
            Ok(unit) => unicode_value = unit as u32,
            Err(e) => return Err(e)
        };

        if stream.peek_equals("\\u") {
            /* ECMA-404 allows UTF-16 surrogate pairs. In that case, we have to transform the
             * surrogates into a code point.
             * See: https://en.wikipedia.org/wiki/UTF-16#Examples */
            stream.consume(2).unwrap();
            match ParserUnicodeSequence::parse_utf_16_code_unit(stream) {
                Ok(unit) => {
                    unicode_value = ((unicode_value & 0x03FF) << 10) + (unit & 0x03FF) + 0x10000;
                }
                Err(e) => return Err(e)
            };
        }

        return match char::from_u32(unicode_value) {
            Some(result) => Ok(Property {
                number: None,
                string: Some(String::from(result)),
                array: None,
                object: None,
                bool: None,
                is_null: false,
            }),
            None => Err(InvalidEscapeSequence)
        };
    }
}

impl ParserUnicodeSequence {
    /// Parses the next four chars of the `json_stream` into a hexadecimal number aka 'code unit'
    fn parse_utf_16_code_unit(json_stream: &mut JsonStream) -> Result<u32, JsonParserError>
    {
        let mut code_point = 0;

        for exponent in (0..4).rev() {
            /* I really don't want to work with strings for performance reasons */
            let multiplier = 16u32.pow(exponent);

            match json_stream.next() {
                Some(character) => {
                    if ('0'..='9').contains(&character) {
                        code_point += multiplier * (character as u32 - '0' as u32);
                    } else if ('A'..='F').contains(&character) {
                        code_point += multiplier * (character as u32 + 10 - 'A' as u32);
                    } else if ('a'..='f').contains(&character) {
                        code_point += multiplier * (character as u32 + 10 - 'a' as u32);
                    }
                }
                None => return Err(UnexpectedEndOfData)
            };
        }

        return Ok(code_point);
    }
}
use crate::constants::token::{
    ESC_SEQUENCE_CHARACTER, ESC_UNICODE_IDENTIFIER, ESC_UNICODE_SEQUENCE_CHARACTER,
    NUM_HEX_A_CAPITAL, NUM_HEX_A_SMALL, NUM_HEX_F_CAPITAL, NUM_HEX_F_SMALL, NUM_NINE, NUM_ZERO,
};
use crate::errors::json_parser_error::JsonParserError::{
    InvalidEscapeSequence, InvalidEscapeSequenceOpeningToken, InvalidEscapeSequenceToken,
    UnexpectedEndOfData, UnknownToken
};
use crate::errors::json_parser_error::JsonParserError;
use crate::structures::json_stream::JsonStream;
use crate::traits::escape_sequence_parser::EscapeSequenceParser;

pub struct ParserUnicodeSequence {}

impl EscapeSequenceParser for ParserUnicodeSequence {
    fn parse(stream: &mut JsonStream) -> Result<char, JsonParserError> {
        stream.skip_whitespaces();

        match stream.next() {
            Some(ESC_SEQUENCE_CHARACTER) => (),
            Some(token) => return Err(InvalidEscapeSequenceOpeningToken(token)),
            None => return Err(UnexpectedEndOfData),
        }

        match stream.next() {
            Some(ESC_UNICODE_SEQUENCE_CHARACTER) => (),
            Some(token) => return Err(InvalidEscapeSequenceToken(token)),
            None => return Err(UnexpectedEndOfData)
        };

        let mut unicode_value;

        match ParserUnicodeSequence::parse_utf_16_code_unit(stream) {
            Ok(unit) => unicode_value = unit as u32,
            Err(e) => return Err(e)
        };

        if stream.peek_equals(ESC_UNICODE_IDENTIFIER) {
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

        match char::from_u32(unicode_value) {
            Some(result) => Ok(result),
            None => Err(InvalidEscapeSequence)
        }
    }
}

impl ParserUnicodeSequence {
    /// Parses the next four chars of the `json_stream` into a hexadecimal number aka 'code unit'
    fn parse_utf_16_code_unit(json_stream: &mut JsonStream) -> Result<u32, JsonParserError>
    {
        let mut code_unit = 0;

        for exponent in (0..4).rev() {
            /* I really don't want to work with strings for performance reasons */
            let multiplier = 16u32.pow(exponent);

            code_unit += multiplier * match json_stream.next() {
                Some(c @ NUM_ZERO..=NUM_NINE) => c as u32 - NUM_ZERO as u32,
                Some(c @ NUM_HEX_A_SMALL..=NUM_HEX_F_SMALL) => c as u32 + 10 - NUM_HEX_A_SMALL as u32,
                Some(c @ NUM_HEX_A_CAPITAL..=NUM_HEX_F_CAPITAL) => c as u32 + 10 - NUM_HEX_A_CAPITAL as u32,
                Some(token) => return Err(UnknownToken(token)),
                None => return Err(UnexpectedEndOfData),
            };
        }

        Ok(code_unit)
    }
}
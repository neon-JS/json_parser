use crate::constants::token::{
    ESC_BACKSPACE, ESC_CARRIAGE_RETURN, ESC_FORM_FEED, ESC_LINE_FEED, ESC_QUOTATION_MARK,
    ESC_REVERSE_SOLIDUS, ESC_SEQUENCE_CHARACTER, ESC_SOLIDUS, ESC_TABULATION,
};
use crate::errors::json_parser_error::JsonParserError::{
    InvalidEscapeSequenceOpeningToken, InvalidEscapeSequenceToken, UnexpectedEndOfData,
};
use crate::errors::json_parser_error::JsonParserError;
use crate::structures::json_stream::JsonStream;
use crate::traits::escape_sequence_parser::EscapeSequenceParser;

pub struct ParserEscapeSequence {}

impl EscapeSequenceParser for ParserEscapeSequence {
    fn parse(stream: &mut JsonStream) -> Result<char, JsonParserError> {
        stream.skip_whitespaces();

        match stream.next() {
            Some(ESC_SEQUENCE_CHARACTER) => (),
            Some(token) => return Err(InvalidEscapeSequenceOpeningToken(token)),
            None => return Err(UnexpectedEndOfData),
        }

        match stream.next() {
            Some(ESC_CARRIAGE_RETURN) => Ok('\r'),
            Some(ESC_REVERSE_SOLIDUS) => Ok('\\'),
            Some(ESC_QUOTATION_MARK) => Ok('"'),
            Some(ESC_TABULATION) => Ok('\t'),
            Some(ESC_BACKSPACE) => Ok('\x08'),
            Some(ESC_FORM_FEED) => Ok('\x0c'),
            Some(ESC_LINE_FEED) => Ok('\n'),
            Some(ESC_SOLIDUS) => Ok('/'),
            Some(token) => Err(InvalidEscapeSequenceToken(token)),
            None => Err(UnexpectedEndOfData)
        }
    }
}
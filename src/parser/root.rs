use crate::constants::token::{
    ARRAY_BRACKET_OPEN, IDENTIFIER_BOOL_FALSE, IDENTIFIER_BOOL_TRUE, IDENTIFIER_NULL, NUM_EIGHT,
    NUM_FIVE, NUM_FOUR, NUM_MINUS, NUM_NINE, NUM_ONE, NUM_SEVEN, NUM_SIX, NUM_THREE, NUM_TWO,
    NUM_ZERO, OBJECT_BRACKET_OPEN, STRING_START,
};
use crate::errors::json_parser_error::JsonParserError::{UnexpectedEndOfData, UnknownToken};
use crate::traits::parser::Parser;
use crate::errors::json_parser_error::JsonParserError;
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;
use crate::parser::array::ParserArray;
use crate::parser::bool::ParserBool;
use crate::parser::null::ParserNull;
use crate::parser::number::ParserNumber;
use crate::parser::object::ParserObject;
use crate::parser::string::ParserString;

pub struct ParserRoot {}

impl Parser for ParserRoot {
    fn parse(stream: &mut JsonStream) -> Result<Property, JsonParserError> {
        stream.skip_whitespaces();

        match stream.peek() {
            Some(ARRAY_BRACKET_OPEN) => ParserArray::parse(stream),
            Some(IDENTIFIER_BOOL_TRUE | IDENTIFIER_BOOL_FALSE) => ParserBool::parse(stream),
            Some(IDENTIFIER_NULL) => ParserNull::parse(stream),
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
            ) => ParserNumber::parse(stream),
            Some(OBJECT_BRACKET_OPEN) => ParserObject::parse(stream),
            Some(STRING_START) => ParserString::parse(stream),
            Some(token) => Err(UnknownToken(token)),
            None => Err(UnexpectedEndOfData),
        }
    }
}
use crate::traits::parser::Parser;
use crate::errors::json_parser_error::JsonParserError;
use crate::errors::json_parser_error::JsonParserError::{UnexpectedEndOfData, UnknownToken};
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;
use crate::parser::array::{ParserArray, ARRAY_OPENING_BRACKET};
use crate::parser::bool::ParserBool;
use crate::parser::null::ParserNull;
use crate::parser::number::{ParserNumber, MINUS};
use crate::parser::object::{ParserObject, OBJECT_OPENING_BRACKET};
use crate::parser::string::{ParserString, DOUBLE_QUOTES};

pub struct ParserRoot {}

impl Parser for ParserRoot {
    fn parse(stream: &mut JsonStream) -> Result<Property, JsonParserError> {
        stream.skip_whitespaces();

        return match stream.peek() {
            Some(ARRAY_OPENING_BRACKET) => ParserArray::parse(stream),
            Some('t')
            | Some('f') => ParserBool::parse(stream),
            Some('n') => ParserNull::parse(stream),
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
            | Some('9') => ParserNumber::parse(stream),
            Some(OBJECT_OPENING_BRACKET) => ParserObject::parse(stream),
            Some(DOUBLE_QUOTES) => ParserString::parse(stream),
            Some(token) => Err(UnknownToken(token)),
            None => Err(UnexpectedEndOfData),
        };
    }
}
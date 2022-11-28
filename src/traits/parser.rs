use crate::errors::json_parser_error::JsonParserError;
use crate::structures::json_stream::JsonStream;
use crate::structures::property::Property;

pub trait Parser {
    fn parse(stream: &mut JsonStream) -> Result<Property, JsonParserError>;
}
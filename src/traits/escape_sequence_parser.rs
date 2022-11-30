use crate::errors::json_parser_error::JsonParserError;
use crate::structures::json_stream::JsonStream;

pub trait EscapeSequenceParser {
    fn parse(stream: &mut JsonStream) -> Result<char, JsonParserError>;
}
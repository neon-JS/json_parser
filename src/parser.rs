use crate::property::Property;

pub trait Parser {
    // TODO: Use Chars or some other structure to prevent converting in each parser
    fn parse<'a>(&mut self, stream: &'a str) -> Result<(Property, &'a str), String>; // TODO: Real error!
}
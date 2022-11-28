use crate::traits::parser::Parser;
use crate::parser::root::ParserRoot;
use crate::structures::json_stream::JsonStream;

mod parser {
    pub mod array;
    pub mod bool;
    pub mod null;
    pub mod number;
    pub mod object;
    pub mod string;
    pub mod root;
}

mod traits {
    pub mod parser;
}

mod structures {
    pub mod json_stream;
    pub mod property;
}

mod errors {
    pub mod json_parser_error;
    pub mod json_stream_error;
}

fn main() {
    let mut x = JsonStream::create("{\"a\": [1, {\"b\": null}], \"b\": true}");

    let res = ParserRoot::parse(&mut x);
    let r2 = res.unwrap();

    print!("{}", r2);
}

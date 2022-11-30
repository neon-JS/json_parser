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

    pub mod sequences {
        pub mod escape_sequence;
        pub mod unicode_sequence;
    }
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

mod constants {
    pub mod token;
}

fn main() {
    let mut x = JsonStream::create("\"x\\uD834\\uDD1E\\u002Fy\"");

    let res = ParserRoot::parse(&mut x);
    let r2 = res.unwrap();

    print!("{}", r2.string.unwrap());
}

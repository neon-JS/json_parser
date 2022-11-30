use crate::traits::parser::Parser;
use crate::parser::root::ParserRoot;
use crate::structures::json_stream::JsonStream;

mod constants {
    pub mod token;
}

mod errors {
    pub mod json_parser_error;
    pub mod json_stream_error;
}

mod parser {
    pub mod sequences {
        pub mod escape_sequence;
        pub mod unicode_sequence;
    }

    pub mod array;
    pub mod bool;
    pub mod null;
    pub mod number;
    pub mod object;
    pub mod root;
    pub mod string;
}

mod structures {
    pub mod json_stream;
    pub mod property;
}

mod traits {
    pub mod escape_sequence_parser;
    pub mod parser;
}

fn main() {
    let mut x = JsonStream::create("\"x\\uD834\\uDD1E\\u002Fy\"");

    let res = ParserRoot::parse(&mut x);
    let r2 = res.unwrap();

    print!("{}", r2.string.unwrap());
}

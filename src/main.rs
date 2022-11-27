use crate::definitions::parser::Parser;
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

mod definitions {
    pub mod parser;
}

mod structures {
    pub mod json_stream;
    pub mod property;
}

fn main() {
    let mut x = JsonStream::create("{\"a\": [1, {\"b\": null}], \"b\": true}");

    let res = ParserRoot::parse(&mut x);
    let r2 = res.unwrap();

    print!("{}", r2);
}

use std::fmt;
use crate::constants::token::{TABULATION, WHITESPACE};
use crate::errors::json_stream_error::JsonStreamError;
use crate::errors::json_stream_error::JsonStreamError::OutOfRange;

pub struct JsonStream<'a> {
    stream: &'a [u8],
    pointer: usize,
    size: usize,
}

impl JsonStream<'_> {
    pub fn peek(&self) -> Option<char> {
        if self.pointer == self.size {
            return None;
        }

        Some(self.stream[self.pointer] as char)
    }

    pub fn peek_equals(&self, compare: &str) -> bool
    {
        let mut pointer = self.pointer;
        for &character in compare.as_bytes() {
            if pointer == self.size {
                return false;
            }

            if character != self.stream[pointer] {
                return false;
            }
            pointer += 1;
        }

        true
    }

    pub fn next(&mut self) -> Option<char> {
        if self.pointer == self.size {
            return None;
        }

        let result = Some(self.stream[self.pointer] as char);
        self.pointer += 1;

        result
    }

    pub fn skip_whitespaces(&mut self) {
        loop {
            if self.pointer == self.size {
                return;
            }

            if self.stream[self.pointer] as char == WHITESPACE {
                self.pointer += 1;
                continue;
            }

            if self.stream[self.pointer] as char == TABULATION {
                self.pointer += 1;
                continue;
            }

            return;
        }
    }

    pub fn consume(&mut self, count: usize) -> Result<(), JsonStreamError> {
        if (self.pointer + count) > self.size {
            return Err(OutOfRange);
        }

        self.pointer += count;

        Ok(())
    }

    pub fn create(json: &str) -> JsonStream {
        JsonStream {
            stream: json.as_bytes(),
            pointer: 0,
            size: json.len(),
        }
    }
}

impl fmt::Display for JsonStream<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stream = String::from_utf8(Vec::from(self.stream)).unwrap();

        write!(f, "Pointer: {}, Size: {}, Stream: '{}'", self.pointer, self.size, stream)
    }
}
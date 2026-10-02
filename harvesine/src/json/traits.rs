use std::collections::HashMap;

pub trait Deserialize: Sized {
    fn deserialize(des: Deserializer, input: &str) -> Result<Self, String>
    where
        Self: Sized;
}

pub enum JsonValue {
    Null,
    String(String),
    Bool(bool),
    Number(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

pub struct Deserializer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Deserializer<'a> {
    pub fn new(input: &'a [u8]) -> Deserializer<'a> {
        Deserializer { input, pos: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    fn next(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.pos += 1;
        Some(byte)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.pos += 1;
        }
    }
}

// {"pairs": [{"x0": 32.0, "y0": 48.3}, {"x0": 81.9, "y0": 75.4}]}

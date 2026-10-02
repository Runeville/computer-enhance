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

    fn parse_string(&mut self) -> Result<String, String> {
        if self.next() != Some(b'"') {
            return Err("Expected '\"'".into());
        }
        let start = self.pos;

        while let Some(byte) = self.peek() {
            if byte == b'"' {
                let slice = &self.input[start..self.pos];
                self.pos += 1;
                return String::from_utf8(slice.to_vec()).map_err(|_| "invalid UTF-8".into());
            }

            self.pos += 1;
        }

        Err("unterminated string".into())
    }

    fn parse_number(&self) -> Result<f64, String> {
        todo!()
    }

    fn parse_object(&self) -> Result<JsonValue, String> {
        todo!()
    }

    fn parse_array(&self) -> Result<JsonValue, String> {
        todo!()
    }

    fn parse_bool(&self) -> Result<bool, String> {
        todo!()
    }

    fn parse_null(&self) -> Result<(), String> {
        if self.input.get(self.pos..self.pos + 4) == Some(b"null") {
            self.pos += 4;
            Ok(())
        } else {
            Err("expected 'null'".into())
        }
    }

    pub fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();

        match self.peek() {
            Some(b'"') => {
                let value = self.parse_string()?;
                Ok(JsonValue::String(value))
            }
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b't') | Some(b'f') => {
                let value = self.parse_bool()?;
                Ok(JsonValue::Bool(value))
            }
            Some(b'n') => {
                self.parse_null()?;
                Ok(JsonValue::Null)
            }

            Some(b'-' | b'0'..=b'9') => {
                let value = self.parse_number()?;
                Ok(JsonValue::Number(value))
            }

            Some(c) => Err(format!(
                "unexpected character '{}' at position {}",
                c as char, self.pos
            )),

            None => Err("Unexpected end of line".to_string()),
        }
    }
}

// {"pairs": [{"x0": 32.0, "y0": 48.3}, {"x0": 81.9, "y0": 75.4}]}

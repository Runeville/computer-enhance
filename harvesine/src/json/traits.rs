use std::collections::HashMap;

pub trait Deserialize: Sized {
    fn deserialize(value: JsonValue) -> Result<Self, String>;
}

impl Deserialize for String {
    fn deserialize(value: JsonValue) -> Result<Self, String> {
        match value {
            JsonValue::String(value) => Ok(value),
            _ => Err("Expected string".to_string()),
        }
    }
}

impl Deserialize for f32 {
    fn deserialize(value: JsonValue) -> Result<Self, String> {
        match value {
            JsonValue::Number(value) => Ok(value),
            _ => Err("Expected number".to_string()),
        }
    }
}

impl<T: Deserialize> Deserialize for Vec<T> {
    fn deserialize(value: JsonValue) -> Result<Self, String> {
        match value {
            JsonValue::Array(values) => values.into_iter().map(T::deserialize).collect(),
            _ => Err("Expected array".to_string()),
        }
    }
}

// Deserializer
#[derive(Clone)]
pub enum JsonValue {
    Null,
    String(String),
    Bool(bool),
    Number(f32),
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

    fn parse_number(&mut self) -> Result<f32, String> {
        self.skip_whitespace();

        let is_negative = self.peek() == Some(b'-');
        if is_negative {
            self.next();
        }

        let mut result = 0.0;

        while let Some(b'0'..=b'9') = self.peek() {
            result = result * 10.0 + (self.next().unwrap() - b'0') as f32;
        }

        if self.peek() == Some(b'.') {
            self.next();

            let mut fraction = 0.1;
            while let Some(b'0'..=b'9') = self.peek() {
                result += (self.next().unwrap() - b'0') as f32 * fraction;
                fraction /= 10.0;
            }
        }

        if is_negative {
            result = -result
        }

        Ok(result)
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        self.next(); // {

        let mut object = HashMap::new();

        self.skip_whitespace();

        if self.peek() == Some(b'}') {
            self.next();
            return Ok(JsonValue::Object(object));
        }

        loop {
            self.skip_whitespace();

            let key = self.parse_string()?;

            self.skip_whitespace();

            if self.next() != Some(b':') {
                return Err("Expected ':'".into());
            }

            let value = self.parse_value()?;
            object.insert(key, value);

            self.skip_whitespace();

            match self.next() {
                Some(b',') => {}
                Some(b'}') => break,
                _ => return Err("Expected ',' or '}'".into()),
            }
        }

        Ok(JsonValue::Object(object))
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        self.next(); // Skip the '['

        let mut values = Vec::new();
        loop {
            self.skip_whitespace();

            if self.peek() == Some(b']') {
                self.next();
                break;
            }

            values.push(self.parse_value()?);

            self.skip_whitespace();

            match self.peek() {
                Some(b',') => {
                    self.next();
                }
                Some(b']') => {
                    self.next();
                    break;
                }
                _ => {
                    return Err("Expected ',' or ']'".into());
                }
            }
        }

        Ok(JsonValue::Array(values))
    }

    fn parse_bool(&mut self) -> Result<bool, String> {
        if self.input.get(self.pos..self.pos + 4) == Some(b"true") {
            self.pos += 4;
            Ok(true)
        } else if self.input.get(self.pos..self.pos + 5) == Some(b"false") {
            self.pos += 5;
            Ok(false)
        } else {
            Err("expected bool".into())
        }
    }

    fn parse_null(&mut self) -> Result<(), String> {
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

use super::{Deserialize, Deserializer};

pub fn parse_json<T>(input: &str) -> Result<T, String>
where
    T: Deserialize,
{
    T::deserialize(Deserializer::new(input.as_bytes()), input)
}

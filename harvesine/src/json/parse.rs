use super::{Deserialize, Deserializer};

pub fn parse_json<T>(input: &str) -> Result<T, String>
where
    T: Deserialize,
{
    let mut des = Deserializer::new(input.as_bytes());
    let value = des.parse_value()?;

    T::deserialize(value)
}

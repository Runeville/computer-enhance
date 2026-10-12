use rand::{RngExt, prelude::IndexedRandom};
use serde::Serialize;

use crate::json::{Deserialize, JsonValue};

#[derive(Debug, Serialize)]
pub struct Pair {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

#[derive(Debug, Serialize)]
pub struct Input {
    pairs: Vec<Pair>,
}

impl Deserialize for Input {
    fn deserialize(value: &JsonValue) -> Result<Self, String> {
        match value {
            JsonValue::Object(object) => {
                let pairs_value = object.get("pairs").ok_or("Missing pairs")?;
                let pairs = Vec::<Pair>::deserialize(pairs_value)?;
                Ok(Input { pairs })
            }
            _ => Err("Expected object".to_string()),
        }
    }
}

impl Deserialize for Pair {
    fn deserialize(value: &JsonValue) -> Result<Self, String> {
        match value {
            JsonValue::Object(object) => Ok(Pair {
                x0: f32::deserialize(object.get("x0").ok_or("Missing x0")?)?,
                y0: f32::deserialize(object.get("y0").ok_or("Missing y0")?)?,
                x1: f32::deserialize(object.get("x1").ok_or("Missing x1")?)?,
                y1: f32::deserialize(object.get("y1").ok_or("Missing y1")?)?,
            }),
            _ => Err("Expected object".to_string()),
        }
    }
}

pub fn generate_pairs_uniform(n_pairs: usize) -> Input {
    let mut rng = rand::rng();

    let pairs = (0..n_pairs)
        .map(|_| Pair {
            x0: rng.random_range(-180.0..=180.0),
            y0: rng.random_range(-90.0..=90.0),
            x1: rng.random_range(-180.0..=180.0),
            y1: rng.random_range(-90.0..=90.0),
        })
        .collect();
    Input { pairs }
}

pub fn generate_pairs_clustered(n_pairs: usize) -> Input {
    const N_CLUSTERS: usize = 16;
    if n_pairs < N_CLUSTERS {
        return generate_pairs_uniform(n_pairs);
    }

    let mut rng = rand::rng();
    let clusters = (0..N_CLUSTERS)
        .map(|_| {
            let x0: f32 = rng.random_range(-180.0..=180.0);
            let x1 = rng.random_range(-180.0..=180.0);
            let y0: f32 = rng.random_range(-90.0..=90.0);
            let y1 = rng.random_range(-90.0..=90.0);
            (x0.min(x1)..=x0.max(x1), y0.min(y1)..=y0.max(y1))
        })
        .collect::<Vec<_>>();

    let pairs = (0..n_pairs)
        .map(|_| {
            let cluster = clusters.choose(&mut rng).unwrap();
            Pair {
                x0: rng.random_range(cluster.0.clone()),
                y0: rng.random_range(cluster.1.clone()),
                x1: rng.random_range(cluster.0.clone()),
                y1: rng.random_range(cluster.1.clone()),
            }
        })
        .collect();

    Input { pairs }
}

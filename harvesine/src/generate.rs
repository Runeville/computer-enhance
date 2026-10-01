use rand::{RngExt, prelude::IndexedRandom};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pair {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    pairs: Vec<Pair>,
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

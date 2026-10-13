use crate::generate::Input;

const R: f32 = 6371.0;

fn calculate_distance(x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
    let dy = (y1 - y0).to_radians();
    let dx = (x1 - x0).to_radians();
    let y0 = y0.to_radians();
    let y1 = y1.to_radians();

    let root_term = (dy / 2.0).sin().powi(2) + y0.cos() * y1.cos() * (dx / 2.0).sin().powi(2);

    2.0 * R * root_term.sqrt().asin()
}

pub fn calculate_average_harvesine_distance(input: Input) -> f32 {
    let n = input.pairs.len() as f32;
    let mut sum = 0.0;
    for pair in input.pairs {
        sum += calculate_distance(pair.x0, pair.y0, pair.x1, pair.y1);
    }

    sum / n
}

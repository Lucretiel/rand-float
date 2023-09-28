use std::iter;

use rand::prelude::*;

fn main() {
    let mut rng = rand::rngs::StdRng::from_entropy();

    let total = 100_000_000;

    let (min, sum, max) = iter::repeat_with(|| rng.gen()).take(total).fold(
        (f64::INFINITY, 0.0f64, f64::NEG_INFINITY),
        |(min, sum, max), x: f64| {
            (
                if min < x { min } else { x },
                sum + x,
                if max > x { max } else { x },
            )
        },
    );

    let avg = sum / (total as f64);

    println!("min: {min}\navg: {avg}\nmax: {max}")
}

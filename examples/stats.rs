use rand::prelude::*;
use rand_float::ImprovedFloat;

fn main() {
    let rng = rand::rngs::StdRng::from_entropy();

    let total = 100_000_000;

    let (min, sum, max) = rng.sample_iter(ImprovedFloat).take(total).fold(
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

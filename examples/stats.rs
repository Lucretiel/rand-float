use std::iter;

use rand::prelude::*;
use rand_float::rand_float;

fn main() {
    let mut rng = rand::rngs::StdRng::from_entropy();

    let total = 100_000_000;

    let (min, sum, max) = iter::repeat_with(|| rand_float(&mut rng)).take(total).fold(
        (f64::INFINITY, 0.0f64, f64::NEG_INFINITY),
        |(min, sum, max), x| {
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

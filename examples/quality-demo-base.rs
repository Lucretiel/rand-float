use std::iter;

use rand::prelude::*;

fn main() {
    let mut rng = StdRng::from_entropy();

    // The idea is that we want to show that the least-significant-bits of very small
    // floats are usually 0, so we can do a bit better by filling them all in

    iter::repeat_with(|| rng.gen())
        .filter(|&value| value < 0.0000001)
        .take(10)
        .for_each(|value: f64| println!("{value}: 0x{bits:X}", bits = value.to_bits()))
}

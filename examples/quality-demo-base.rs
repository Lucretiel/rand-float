use rand::{distributions, prelude::*};

fn main() {
    let rng = StdRng::from_entropy();

    // The idea is that we want to show that the least-significant-bits of very small
    // floats are usually 0, so we can do a bit better by filling them all in

    rng.sample_iter(distributions::Standard)
        .filter(|&value| value < 0.0000001)
        .take(10)
        .for_each(|value: f64| println!("{value}: 0x{bits:X}", bits = value.to_bits()))
}

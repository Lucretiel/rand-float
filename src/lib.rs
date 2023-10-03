use rand::prelude::*;

pub struct ImprovedFloat;

impl Distribution<f64> for ImprovedFloat {
    #[inline]
    #[must_use]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let bits: u64 = rng.next_u64();

        // Technically we could loop here and collect up to 1022 leading 0s, for
        // ultra-high precision random tiny floats, but we assume that for
        // practical purposes we'll never see even 32 consecutive 0 bits.
        if bits == 0 {
            return 0.0;
        }

        let offset = bits.leading_zeros();
        let biased_exponent = (1022 - offset) as u64;

        // We need to fill the float's mantissa with 52 random bits. It's
        // possible we already have that in our random bits, though we need to
        // make sure we throw away the leading 1, which is by definition not
        // random.
        //
        // The mantissa will be the 52 least significant bits in this value.
        // Theoretically we could do something complex like generate a sequence
        // of u8 or u32 until we have just enough bits, but we're gonna assume RNGs
        // are fast at making u64 and so do the easy thing.
        let repr = if 64 - (offset + 1) >= 52 {
            bits
        } else {
            rng.next_u64()
        };

        // Clear the top 12 bits
        let repr = repr << 12;
        let repr = repr >> 12;

        // Insert the exponent
        let repr = repr | (biased_exponent << 52);

        f64::from_bits(repr)
    }
}

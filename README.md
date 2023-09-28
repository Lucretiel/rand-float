# rand-float

This repository contains a proposed algorithm for higher-quality random floats,for `rand`. The current algorithm is (more-or-less) based on unconditionally using 52 bits of a `u64` with fixed precision; this means that if those bits have a lot of leading zeros, the output floats end up with a lot of least-significant 0 bits. This property is demonstrated in the `quality-demo-base` example, with the improved quality demonstrated in `quality-demo`.

The new algorithm performs only slightly slower than the old one; it requires a second `rng` int in the event that the first one had a lot of leading zeroes and we have more room for significant bits in the tail. This second random value is only required (1/(2^12)) times. Performance demos (generate 100M random floats and compute min / mean / max) are included as `stats` (for the new algorithm) and `stats-base` (for the old one); run them with `hyperfine` or your other bencher of choice.

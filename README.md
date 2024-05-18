# rand-float

This repository contains a proposed algorithm for higher-quality random floats,for `rand`. The current algorithm is (more-or-less) based on unconditionally using 52 bits of a `u64` with fixed precision; this means that if those bits have a lot of leading zeros, the output floats end up with a lot of least-significant 0 bits. This property is demonstrated in the `quality-demo-base` example, with the improved quality demonstrated in `quality-demo`.

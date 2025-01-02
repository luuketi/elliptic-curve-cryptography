use num_bigint::BigInt;
use crate::s256_point::S256Point;

#[derive(Clone, Debug)]
pub struct Signature {
    r: BigInt,
    s: BigInt
}

impl Signature {
    pub fn new(r: BigInt, s: BigInt) -> Self {
        Self {r, s}
    }
    pub fn s_inv(self, exponent :BigInt, n: BigInt ) -> BigInt {
        self.s.modpow(&exponent, &n.clone())
    }
    pub fn v(self, s_inv:BigInt, n: BigInt) -> BigInt {
        self.r * s_inv % n
    }
    pub fn verify_point(self, point: S256Point) -> bool {
        point.compare_x(self.r)
    }
}
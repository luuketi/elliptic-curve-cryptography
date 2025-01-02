use std::ops::{Add, Mul};
use num_bigint::{BigInt, Sign};
use crate::point::Point;
use hex;
use crate::field_element::FieldElement;
use crate::s256_field::S256Field;
use crate::signature::Signature;

pub(crate) const N: &str = "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";
pub(crate) const GX: &str = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
pub(crate) const GY: &str = "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

pub struct S256Point {
    point: Point
}

impl S256Point {
    pub fn new(x: Option<BigInt>, y: Option<BigInt>) -> Self {
        let a = S256Field::new(0.into());
        let b = S256Field::new(7.into());
        if Some(x.clone()) != None && Some(y.clone()) != None {
            let new_x : FieldElement = S256Field::new(x.unwrap()).into();
            let new_y : FieldElement = S256Field::new(y.unwrap()).into();
            Self { point: Point::from_field_element(new_x.into(), new_y.into(), a.into(), b.into()) }
        } else {
            Self { point: Point::from_field_element(None, None, a.into(), b.into()) }
        }
    }

    pub fn verify(self, z: BigInt, signature: Signature) -> bool {
        let n = BigInt::from_bytes_be(Sign::Plus, &hex::decode(N).unwrap());
        let exp = n.clone() - BigInt::from(2);
        let s_inv = signature.clone().s_inv(exp, n.clone());
        let u = z * s_inv.clone() % n.clone();
        let v = signature.clone().v(s_inv, n);
        let g = S256Point::new(
            Some(BigInt::from_bytes_be(Sign::Plus, &hex::decode(GX).unwrap())),
            Some(BigInt::from_bytes_be(Sign::Plus, &hex::decode(GY).unwrap())));
        let total = g * u + self * v;
        signature.verify_point(total)
    }

    pub fn compare_x(self, r: BigInt) -> bool {
        self.point.compare_x(r)
    }
}

impl Add for S256Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        S256Point{ point: self.point + rhs.point }
    }
}


impl Mul<BigInt> for S256Point {
    type Output = Self;

    fn mul(self, coefficient: BigInt) -> Self::Output {
        let n = BigInt::from_bytes_be(Sign::Plus, &hex::decode(N).unwrap());
        let coef : BigInt = coefficient % n;
        S256Point{ point: self.point * coef }
    }
}

impl PartialEq for S256Point {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}


#[cfg(test)]
mod tests {
    use num_bigint::Sign;
    use super::*;

    #[test]
    fn verify_signature() {
        let z = BigInt::from_bytes_be(Sign::Plus, &hex::decode("bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423").unwrap());
        let r = BigInt::from_bytes_be(Sign::Plus, &hex::decode("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6").unwrap());
        let s = BigInt::from_bytes_be(Sign::Plus, &hex::decode("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec").unwrap());
        let signature = Signature::new(r, s);
        let px = BigInt::from_bytes_be(Sign::Plus, &hex::decode("04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574").unwrap());
        let py = BigInt::from_bytes_be(Sign::Plus, &hex::decode("82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4").unwrap());
        let point = S256Point::new(Some(px), Some(py));

        assert!(point.verify(z, signature));
    }
}
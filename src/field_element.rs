use num_bigint::BigInt;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};


#[derive(Debug, Eq, Clone)]
pub struct FieldElement {
    number : BigInt,
    prime : BigInt,
}

impl FieldElement {
    pub fn from_i32(number: i32, prime: i32) -> Self {
        Self::new( BigInt::from(number), BigInt::from(prime) )
    }

    pub fn new(number: BigInt, prime: BigInt) -> Self {
        if number >= prime || number < BigInt::from(0) {
            panic!("Num {} not in field range 0 to {}", number, prime - 1);
        }
        Self { number, prime }
    }

    pub fn pow(self, exponent: BigInt) -> Self {
        let prime = self.prime.clone() - 1;
        let n = exponent.modpow(&BigInt::from(1), &prime);
        let number = self.number.modpow(&n, &self.prime);
        Self { number, prime: self.prime }
    }

    pub fn number(self) -> BigInt {
        return self.number
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.number)
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.prime == other.prime
    }
}

impl PartialEq<BigInt> for FieldElement {
    fn eq(&self, other: &BigInt) -> bool {
        self.number == *other
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields")
        }
        let number = (self.number + other.number).modpow(&BigInt::from(1), &self.prime);
        Self { number, prime: self.prime }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot sub two numbers in different Fields")
        }
        let number = (self.number - other.number).modpow(&BigInt::from(1), &self.prime);
        Self { number, prime: self.prime }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot mul two numbers in different Fields")
        }
        let number = (self.number * other.number).modpow(&BigInt::from(1), &self.prime);
        Self { number, prime: self.prime }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot div two numbers in different Fields")
        }
        let other_number = other.pow(self.prime.clone() - 2).number;
        let number = (self.number * other_number).modpow(&BigInt::from(1), &self.prime);
        Self { number, prime: self.prime }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_field_elements() {
        let a = FieldElement::new(BigInt::from(7), BigInt::from(13));
        let b = FieldElement::new(BigInt::from(6), BigInt::from(13));
        println!("{}", a);
        println!("{}", b);

        assert_eq!(a, a);
        assert_ne!(a, b);
    }

    #[test]
    fn add_field_elements() {
        let a = FieldElement::new(BigInt::from(7), BigInt::from(13));
        let b = FieldElement::new(BigInt::from(12), BigInt::from(13));
        let c = FieldElement::new(BigInt::from(6), BigInt::from(13));

        assert_eq!(a+b, c);
    }

    #[test]
    fn sub_field_elements() {
        let a = FieldElement::new(BigInt::from(11), BigInt::from(19));
        let b = FieldElement::new(BigInt::from(9), BigInt::from(19));
        let c = FieldElement::new(BigInt::from(2), BigInt::from(19));
        let d = FieldElement::new(BigInt::from(6), BigInt::from(19));
        let e = FieldElement::new(BigInt::from(13), BigInt::from(19));
        let f = FieldElement::new(BigInt::from(12), BigInt::from(19));

        assert_eq!(a-b, c);
        assert_eq!(d-e, f);
    }

    #[test]
    fn mul_field_elements() {
        let a = FieldElement::new(BigInt::from(3), BigInt::from(13));
        let b = FieldElement::new(BigInt::from(12), BigInt::from(13));
        let c = FieldElement::new(BigInt::from(10), BigInt::from(13));

        assert_eq!(a*b, c);
    }

    #[test]
    fn pow_field_elements() {
        let a = FieldElement::new(BigInt::from(3), BigInt::from(13));
        let b = FieldElement::new(BigInt::from(1), BigInt::from(13));

        assert_eq!(a.pow(BigInt::from(3)), b);
    }

    #[test]
    fn div_field_elements() {
        let a = FieldElement::new(BigInt::from(2), BigInt::from(19));
        let b = FieldElement::new(BigInt::from(7), BigInt::from(19));
        let c = FieldElement::new(BigInt::from(3), BigInt::from(19));
        let d = FieldElement::new(BigInt::from(7), BigInt::from(19));
        let e = FieldElement::new(BigInt::from(5), BigInt::from(19));
        let f = FieldElement::new(BigInt::from(9), BigInt::from(19));

        assert_eq!(a/b, c);
        assert_eq!(d/e, f);
    }

}
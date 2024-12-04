use num_bigint::BigInt;
use std::fmt;
use std::ops::Add;


#[derive(Debug, Eq)]
pub struct FieldElement {
    number : BigInt,
    prime : BigInt,
}

impl FieldElement {
    pub fn new(number: BigInt, prime: BigInt) -> Self {
        if number >= prime || number < BigInt::from(0) {
            panic!("Num {} not in field range 0 to {}", number, prime - 1);
        }
        Self { number, prime }
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

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields")
        }
        let number = (self.number + other.number) % self.prime.clone();
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
}
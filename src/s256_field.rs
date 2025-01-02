use std::str::FromStr;
use num_bigint::BigInt;
use crate::field_element::FieldElement;


pub struct S256Field {
    field_element: FieldElement
}

impl S256Field {
    pub fn new(number: BigInt) -> Self {
        let p = BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007908834671663").unwrap();
        Self { field_element: FieldElement::new(number, p) }
    }
}

impl From<S256Field> for FieldElement {
    fn from(i: S256Field) -> Self {
        i.field_element
    }
}

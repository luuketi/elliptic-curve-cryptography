use std::ops::{Add, BitAnd, BitAndAssign, Mul};
use num_bigint::{BigInt, ToBigInt};
use bigdecimal::{BigDecimal, One, Zero};
use crate::field_element::FieldElement;

#[derive(Clone, Debug, Eq)]
pub struct Point {
    a: BigInt,
    b: BigInt,
    x: Option<BigInt>,
    y: Option<BigInt>,
}

impl Point {
    pub fn from_field_element(x: Option<FieldElement>, y: Option<FieldElement>, a: FieldElement, b: FieldElement) -> Self {
        if x.clone().is_some() && y.clone().is_some() {
            Self::new(x.unwrap().number().into(), y.unwrap().number().into(), a.number(), b.number())
        } else {
            Self::new(None, None, a.number(), b.number())
        }
    }

    pub fn new(x: Option<BigInt>, y: Option<BigInt>, a: BigInt, b: BigInt) -> Self {
        if x.is_some() && y.is_some() &&
            y.clone().unwrap().pow(2) != x.clone().unwrap().pow(3) + a.clone() * x.clone().unwrap() + b.clone() {
                panic!("'({}, {}) is not on the curve", x.unwrap(), y.unwrap());
        }
        Self { a, b, x, y }
    }

    pub fn compare_x(self, r: BigInt) -> bool {
        self.x.unwrap() == r
    }

}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.x == other.x && self.y == other.y
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, other)
        }
        if self.x.is_none() {
            return other
        }
        if other.x.is_none() {
            return self
        }
        let x1 = self.x.clone().unwrap();
        let x2 = other.x.clone().unwrap();
        let y1 = self.y.clone().unwrap();
        let y2 = other.y.clone().unwrap();

        if self == other {
            if y1 == BigInt::from(0) {
                Self::new(None, None, self.a, self.b)
            } else {
                let s = BigDecimal::from(3 * x1.clone().pow(2) + self.a.clone()) / BigDecimal::from(2 * y1);
                let x : BigDecimal = s.clone() * s.clone() - 2 * x1.clone();
                let y : BigDecimal = s.clone() * (x1 - x.clone()) - self.y.unwrap();
                Self::new(x.to_bigint(), y.to_bigint(), self.a, self.b)
            }
        }
        else if x1 == x2 {
            Self::new(None, None, self.a, self.b)
        } else {
            let s = BigDecimal::from(y2 - y1) / BigDecimal::from(x2.clone() - x1.clone());
            let x = s.clone() * s.clone() - x1.clone() - x2;
            let y = s.clone() * (x1 - x.clone()) - self.y.unwrap();
            Self::new(x.to_bigint(), y.to_bigint(), self.a, self.b)
        }
    }
}

impl Mul<BigInt> for Point {
    type Output = Self;

    fn mul(self, coefficient: BigInt) -> Self::Output {
        let mut coef = coefficient;
        let mut current = self.clone();
        let mut result = Self::new(None, None, self.a, self.b);
        while coef.clone() != BigInt::zero() {
            if coef.clone().bitand( BigInt::one() ) != BigInt::zero() {
                result = result + current.clone();
            }
            current = current.clone() + current.clone();
            coef.bitand_assign( BigInt::one() );
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    #[should_panic]
    fn point_not_in_curve() {
        Point::new(BigInt::from(-1).into(), BigInt::from(-2).into(), BigInt::from(5), BigInt::from(7));
    }

    #[test]
    fn compare_points() {
        let a = Point::new(BigInt::from(-1).into(), BigInt::from(-1).into(), BigInt::from(5), BigInt::from(7));

        assert_eq!(a, a);
    }

    #[test]
    fn add_points() {
        let p1 = Point::new(BigInt::from(-1).into(), BigInt::from(-1).into(), BigInt::from(5), BigInt::from(7));
        let p2 = Point::new(BigInt::from(-1).into(), BigInt::from(1).into(), BigInt::from(5), BigInt::from(7));
        let inf = Point::new(None, None, BigInt::from(5), BigInt::from(7));

        assert_eq!(p1.clone() + inf.clone(), p1);
        assert_eq!(inf.clone() + p2.clone(), p2);
        assert_eq!(p1.clone() + p2.clone(), inf);
    }

    #[test]
    fn add_points_with_x1_notequal_to_x2() {
        let p1 = Point::new(BigInt::from(2).into(), BigInt::from(5).into(), BigInt::from(5), BigInt::from(7));
        let p2 = Point::new(BigInt::from(-1).into(), BigInt::from(-1).into(), BigInt::from(5), BigInt::from(7));
        let p3 = Point::new(BigInt::from(3).into(), BigInt::from(-7).into(), BigInt::from(5), BigInt::from(7));

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn add_points_with_p1_equal_to_p2() {
        let p1 = Point::new(BigInt::from(-1).into(), BigInt::from(-1).into(), BigInt::from(5), BigInt::from(7));
        let p2 = Point::new(BigInt::from(-1).into(), BigInt::from(-1).into(), BigInt::from(5), BigInt::from(7));
        let p3 = Point::new(BigInt::from(18).into(), BigInt::from(77).into(), BigInt::from(5), BigInt::from(7));

        assert_eq!(p1 + p2, p3);
    }

}
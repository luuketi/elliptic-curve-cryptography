use std::ops::Add;
use num_bigint::BigInt;


#[derive(Clone, Debug, Eq)]
pub struct Point {
    a: BigInt,
    b: BigInt,
    x: Option<BigInt>,
    y: Option<BigInt>,
}

impl Point {
    pub fn new(x: Option<BigInt>, y: Option<BigInt>, a: BigInt, b: BigInt) -> Self {
        if x != None && y != None {
            if y.clone().unwrap().pow(2) != x.clone().unwrap().pow(3) + a.clone() * x.clone().unwrap() + b.clone() {
                panic!("'({}, {}) is not on the curve", x.unwrap(), y.unwrap());
            }
        }
        Self { a, b, x, y }
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
        if self.x == None {
            return other
        }
        if other.x == None {
            return self
        }
        if self.x != None && other.x != None && self.x.unwrap() == other.x.unwrap() {
            return Self::new(None, None, self.a, self.b)
        }

        Self {
            a: Default::default(),
            b: Default::default(),
            x: None,
            y: None,
        }
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

}
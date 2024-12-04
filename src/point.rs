use num_bigint::BigInt;

#[derive(Debug, Eq)]
pub struct Point {
    a: BigInt,
    b: BigInt,
    x: BigInt,
    y: BigInt,
}

impl Point {
    pub fn new(x: BigInt, y: BigInt, a: BigInt, b: BigInt) -> Self {
        if y.clone().pow(2) != x.clone().pow(3) + a.clone() * x.clone() + b.clone() {
            panic!("'({}, {}) is not on the curve", x, y);
        }
        Self { a, b, x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.x == other.x && self.y == other.y
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    #[should_panic]
    fn point_not_in_curve() {
        Point::new(BigInt::from(-1), BigInt::from(-2), BigInt::from(5), BigInt::from(7));
    }

    #[test]
    fn compare_points() {
        let a = Point::new(BigInt::from(-1), BigInt::from(-1), BigInt::from(5), BigInt::from(7));

        assert_eq!(a, a);
    }
}
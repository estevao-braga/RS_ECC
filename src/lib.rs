#![allow(dead_code)]

use num_bigint::BigUint;

#[derive(Clone, PartialEq, Debug)]
pub enum Point {
    Coor(BigUint, BigUint),
    Identity,
}

pub struct EllipticCurve {
    // y² = x³ + ax + b
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

struct FiniteField;

impl EllipticCurve {
    pub fn add(&self, c: &Point, d: &Point) -> Point {
        assert!(self.is_on_curve(c), "Point is not on curve");
        assert!(self.is_on_curve(d), "Point is not on curve");
        assert!(c != d, "Points should not be the same");

        match (c, d) {
            (Point::Identity, _) => d.clone(),
            (_, Point::Identity) => c.clone(),
            (Point::Coor(x1, y1), Point::Coor(x2, y2)) => {
                let x3;
                let y3;

                let y1plusy2 = FiniteField::add(y1, y2, &self.p);

                if x1 == x2 && y1plusy2 == BigUint::from(0u32) {
                    return Point::Identity;
                }

                {
                    let y2minusy1 = FiniteField::sub(y2, y1, &self.p);
                    let x2minusx1 = FiniteField::sub(x2, x1, &self.p);
                    let s = FiniteField::div(&y2minusy1, &x2minusx1, &self.p);
                    let s2 = s.modpow(&BigUint::from(2u32), &self.p);
                    let s2minusx1 = FiniteField::sub(&s2, x1, &self.p);
                    x3 = FiniteField::sub(&s2minusx1, x2, &self.p);
                    let x1minusx3 = FiniteField::sub(x1, &x3, &self.p);
                    let sx1minusx3 = FiniteField::mult(&s, &x1minusx3, &self.p);
                    y3 = FiniteField::sub(&sx1minusx3, y1, &self.p);
                }

                Point::Coor(x3, y3)
            }
        }
    }

    pub fn double(&self, c: &Point) -> Point {
        assert!(self.is_on_curve(c), "Point is not on curve");

        match c {
            Point::Identity => Point::Identity,
            Point::Coor(x1, y1) => {
                let x3;
                let y3;

                {
                    let s;
                    {
                        let x1square = x1.modpow(&BigUint::from(2u32), &self.p);
                        let x1square3 = FiniteField::mult(&BigUint::from(3u32), &x1square, &self.p);
                        let numerator = FiniteField::add(&x1square3, &self.a, &self.p);
                        let denominator = FiniteField::mult(&BigUint::from(2u32), y1, &self.p);
                        s = FiniteField::div(&numerator, &denominator, &self.p);
                    }
                    let s2 = s.modpow(&BigUint::from(2u32), &self.p);
                    let s2minusx1 = FiniteField::sub(&s2, x1, &self.p);
                    x3 = FiniteField::sub(&s2minusx1, x1, &self.p);
                    let x1minusx3 = FiniteField::sub(x1, &x3, &self.p);
                    let sx1minusx3 = FiniteField::mult(&s, &x1minusx3, &self.p);
                    y3 = FiniteField::sub(&sx1minusx3, y1, &self.p);
                }

                Point::Coor(x3, y3)
            }
        }
    }

    pub fn scalar_mult(&self, c: &Point, d: &BigUint) -> Point {
        let mut t = c.clone();
        for i in (0..(d.bits() - 1)).rev() {
            t = self.double(&t);
            if d.bit(i) {
                t = self.add(&t, c);
            }
        }
        t
    }

    pub fn is_on_curve(&self, c: &Point) -> bool {
        match c {
            Point::Coor(x, y) => {
                let y2 = y.modpow(&BigUint::from(2u32), &self.p);
                let x3 = x.modpow(&BigUint::from(3u32), &self.p);
                let ax = FiniteField::mult(&self.a, x, &self.p);
                y2 == FiniteField::add(&x3, &FiniteField::add(&ax, &self.b, &self.p), &self.p)
            }
            Point::Identity => true,
        }
    }
}

impl FiniteField {
    pub fn add(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
        assert!(a < p, "{a} >= {p}");
        assert!(b < p, "{b} >= {p}");

        let r = a + b;
        r.modpow(&BigUint::from(1u32), p)
    }

    pub fn mult(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
        assert!(a < p, "{a} >= {p}");
        assert!(b < p, "{b} >= {p}");

        let r = a * b;
        r.modpow(&BigUint::from(1u32), p)
    }

    pub fn sub(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
        assert!(a < p, "{a} >= {p}");
        assert!(b < p, "{b} >= {p}");

        let b_inv = FiniteField::inv_add(b, p);
        FiniteField::add(a, &b_inv, p)
    }

    pub fn div(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
        assert!(a < p, "{a} >= {p}");
        assert!(b < p, "{b} >= {p}");

        let b_inv = FiniteField::inv_mult(b, p);
        FiniteField::mult(a, &b_inv, p)
    }

    fn inv_add(a: &BigUint, p: &BigUint) -> BigUint {
        assert!(a < p, "{a} >= {p}");

        p - a
    }

    fn inv_mult(a: &BigUint, p: &BigUint) -> BigUint {
        assert!(a < p, "{a} >= {p}");

        a.modpow(&(p - BigUint::from(2u32)), p)
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_add() {
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let p = BigUint::from(11u32);
        let r = FiniteField::add(&a, &b, &p);
        assert_eq!(r, BigUint::from(3u32));
    }

    #[test]
    fn test_mult() {
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let p = BigUint::from(31u32);
        let r = FiniteField::mult(&a, &b, &p);
        assert_eq!(r, BigUint::from(9u32));
    }

    #[test]
    fn test_inv_add() {
        let a = BigUint::from(4u32);
        let p = BigUint::from(31u32);
        let r = FiniteField::inv_add(&a, &p);
        assert_eq!(r, BigUint::from(27u32));
        assert_eq!(FiniteField::add(&a, &r, &p), BigUint::from(0u32));
    }

    #[test]
    #[should_panic]
    fn test_inv_add2() {
        let a = BigUint::from(43u32);
        let p = BigUint::from(31u32);
        let r = FiniteField::inv_add(&a, &p);
        assert_eq!(r, BigUint::from(27u32));
    }

    #[test]
    fn test_inv_mult() {
        let a = BigUint::from(4u32);
        let p = BigUint::from(31u32);
        let r = FiniteField::inv_mult(&a, &p);
        assert_eq!(FiniteField::mult(&a, &r, &p), BigUint::from(1u32));
    }

    #[test]
    fn test_ec_point_add() {
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        let p1 = Point::Coor(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Coor(BigUint::from(5u32), BigUint::from(1u32));
        let pr = Point::Coor(BigUint::from(10u32), BigUint::from(6u32));
        assert_eq!(ec.add(&p1, &p2), pr);

        let p1 = Point::Coor(BigUint::from(5u32), BigUint::from(16u32));
        let pr = Point::Identity;
        assert_eq!(ec.add(&p1, &p2), pr);
    }

    #[test]
    fn test_ec_point_double() {
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        let p = Point::Coor(BigUint::from(5u32), BigUint::from(1u32));
        let pr = Point::Coor(BigUint::from(6u32), BigUint::from(3u32));
        assert_eq!(ec.double(&p), pr);
    }

    #[test]
    fn test_ec_point_mult() {
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        let c = Point::Coor(BigUint::from(5u32), BigUint::from(1u32));

        let pr = Point::Coor(BigUint::from(6u32), BigUint::from(3u32));
        let res = ec.scalar_mult(&c, &BigUint::from(2u32));
        assert_eq!(res, pr);

        let pr = Point::Coor(BigUint::from(7u32), BigUint::from(11u32));
        let res = ec.scalar_mult(&c, &BigUint::from(10u32));
        assert_eq!(res, pr);
    }

    #[test]
    fn test_ec_secp256k1() {
        let p = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16,
        )
        .unwrap();

        let n = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
        .unwrap();

        let gx = BigUint::parse_bytes(
            b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        )
        .unwrap();

        let gy = BigUint::parse_bytes(
            b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
            16,
        )
        .unwrap();

        let ec = EllipticCurve {
            a: BigUint::from(0u32),
            b: BigUint::from(7u32),
            p,
        };

        let g = Point::Coor(gx, gy);

        let res = ec.scalar_mult(&g, &n);
        assert_eq!(res, Point::Identity);
    }
}

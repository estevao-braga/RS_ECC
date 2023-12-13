#![allow(dead_code, unused_imports)]

use ec_generic::{EllipticCurve, FiniteField, Point};
use num_bigint::{BigUint, RandBigInt};
use rand::{self, Rng};
use sha256::{digest, try_digest};

struct ECDSA {
    elliptic_curve: EllipticCurve,
    a_gen: Point,
    q_order: BigUint,
}

impl ECDSA {
    pub fn generate_key_pair(&self) -> (BigUint, Point) {
        // Generates: d, B, where B = dA
        let priv_key = self.generate_priv_key();
        let pub_key = self.generate_pub_key(&priv_key);
        (priv_key, pub_key)
    }

    pub fn generate_priv_key(&self) -> BigUint {
        self.generate_random_number_less_than(&self.q_order)
    }

    pub fn generate_pub_key(&self, priv_key: &BigUint) -> Point {
        self.elliptic_curve
            .scalar_mul(&self.a_gen, priv_key)
            .unwrap()
    }

    pub fn generate_random_number_less_than(&self, max: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();
        rng.gen_biguint_range(&BigUint::from(1u32), &max)
    }

    pub fn sign(
        &self,
        hash: &BigUint,
        priv_key: &BigUint,
        k_random: &BigUint,
    ) -> (BigUint, BigUint) {
        // R(x,y) = kA -> take r = x
        // s = (hash(m) + d * r) * k⁻¹ mod q

        assert!(hash < &self.q_order, "Hash is Bigger than the Ec group");
        assert!(
            priv_key < &self.q_order,
            "Private Key is Bigger than the Ec group"
        );
        assert!(k_random < &self.q_order, "'K' is Bigger than the Ec group");

        let r_point = self
            .elliptic_curve
            .scalar_mul(&self.a_gen, k_random)
            .unwrap();

        if let Point::Coor(r, _) = r_point {
            let s = FiniteField::mult(&r, priv_key, &self.q_order).unwrap();
            let s = FiniteField::add(&s, hash, &self.q_order).unwrap();
            let k_inv = FiniteField::inv_mult_prime(&k_random, &self.q_order).unwrap();
            let s = FiniteField::mult(&s, &k_inv, &self.q_order).unwrap();
            return (r, s);
        }
        panic!("The random point R should not be the identity");
    }

    pub fn verification(
        &self,
        hash: &BigUint,
        pub_key: &Point,
        signature: &(BigUint, BigUint),
    ) -> bool {
        assert!(
            hash < &self.q_order,
            "Hash is bigger than the order of the EC group"
        );

        let (r, s) = signature;
        let s_inv = FiniteField::inv_mult_prime(&s, &self.q_order).unwrap();
        let u1 = FiniteField::mult(&s_inv, hash, &self.q_order).unwrap();
        let u2 = FiniteField::mult(&s_inv, r, &self.q_order).unwrap();
        let u1a = self.elliptic_curve.scalar_mul(&self.a_gen, &u1).unwrap();
        let u2b = self.elliptic_curve.scalar_mul(&pub_key, &u2).unwrap();
        let p = self.elliptic_curve.add(&u1a, &u2b).unwrap();

        if let Point::Coor(xp, _) = p {
            return &xp == r;
        }
        panic!("Point P = u1 + u2 cannot be the identity")
    }

    pub fn generate_hash_less_than(&self, message: &str, max: &BigUint) -> BigUint {
        let digest = digest(message);
        let hash_bytes = hex::decode(digest).expect("Could not convert hash to Vec<u8>");
        let hash = BigUint::from_bytes_be(&hash_bytes)
            .modpow(&BigUint::from(1u32), &(max - BigUint::from(1u32)));
        let hash = hash + BigUint::from(1u32);
        hash
    }
}

#[cfg(test)]
mod test {
    use std::hash;

    use super::*;

    #[test]
    fn test_sign_verify() {
        let elliptic_curve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        let a_gen = Point::Coor(BigUint::from(5u32), BigUint::from(1u32));
        let q_order = BigUint::from(19u32);

        let ecdsa = ECDSA {
            elliptic_curve,
            a_gen,
            q_order,
        };

        let priv_key = BigUint::from(7u32);
        let pub_key = ecdsa.generate_pub_key(&priv_key);

        let k_random = BigUint::from(18u32);

        let message = "Bob -> 1 BTC -> Alice";
        let hash = ecdsa.generate_hash_less_than(&message, &ecdsa.q_order);

        let signature = ecdsa.sign(&hash, &priv_key, &k_random);
        let verify_result = ecdsa.verification(&hash, &pub_key, &signature);

        assert!(verify_result, "Verification should sucess");
    }

    #[test]
    fn test_sign_verify_tempered_message() {
        let elliptic_curve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        let a_gen = Point::Coor(BigUint::from(5u32), BigUint::from(1u32));
        let q_order = BigUint::from(19u32);

        let ecdsa = ECDSA {
            elliptic_curve,
            a_gen,
            q_order,
        };

        let priv_key = BigUint::from(7u32);
        let pub_key = ecdsa.generate_pub_key(&priv_key);

        let k_random = BigUint::from(18u32);

        let message = "Bob -> 1 BTC -> Alice";
        let hash = ecdsa.generate_hash_less_than(&message, &ecdsa.q_order);
        let signature = ecdsa.sign(&hash, &priv_key, &k_random);

        let message = "Bob -> 2 BTC -> Alice";
        let hash = ecdsa.generate_hash_less_than(&message, &ecdsa.q_order);
        let verify_result = ecdsa.verification(&hash, &pub_key, &signature);

        assert!(!verify_result, "Verification should fail");
    }

    #[test]
    fn test_sign_verify_tempered_signature() {
        let elliptic_curve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        let a_gen = Point::Coor(BigUint::from(5u32), BigUint::from(1u32));
        let q_order = BigUint::from(19u32);

        let ecdsa = ECDSA {
            elliptic_curve,
            a_gen,
            q_order,
        };

        let priv_key = BigUint::from(7u32);
        let pub_key = ecdsa.generate_pub_key(&priv_key);

        let k_random = BigUint::from(4u32);

        let message = "Bob -> 1 BTC -> Alice";
        let hash = ecdsa.generate_hash_less_than(&message, &ecdsa.q_order);
        let signature = ecdsa.sign(&hash, &priv_key, &k_random);
        let (r, s) = signature;
        let tempered_siganture = (
            (r + BigUint::from(1u32)).modpow(&BigUint::from(1u32), &ecdsa.q_order),
            s,
        );

        let verify_result = ecdsa.verification(&hash, &pub_key, &tempered_siganture);

        assert!(!verify_result, "Verification should fail");
    }
}

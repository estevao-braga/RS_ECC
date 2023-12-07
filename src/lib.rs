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

    pub fn verification(&self, hash: &BigUint, pub_key: &Point, signature: &BigUint) -> bool {
        todo!()
    }
}

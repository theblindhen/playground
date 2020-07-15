use num_bigint::BigInt;
use num_traits::Zero;

pub fn big_ea(a : &BigInt, b : &BigInt) -> BigInt {
    if *b == Zero::zero() {
        a.clone()
    } else {
        let mut r1 = a.clone();
        let mut r2 = b.clone();
        while r2 != Zero::zero() {
            let r = &r1 % &r2;
            r1 = r2;
            r2 = r;
        }
        r1
    }
}

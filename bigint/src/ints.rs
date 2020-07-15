
use num_bigint::BigInt;
use num_traits::Zero;

// This version is the straight way to write Euclidean algorithm so that the
// borrow-checker accepts it
pub fn big_ea_simple(a : &BigInt, b : &BigInt) -> BigInt {
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

// This version loop-unrolls twice to avoid gratuitous use of clone()
pub fn big_ea_fast(a : &BigInt, b : &BigInt) -> BigInt {
    let z = Zero::zero();
    if *b == z {
        a.clone()
    } else {
        let mut r1 = a % b;
        if r1 == z {
            b.clone()
        } else {
            let mut r2 = b % &r1;
            while r2 != z {
                // Avoid a new allocation every iteration if BigInt is clever enough.
                r1 %= &r2; 
                let r = r1;
                r1 = r2;
                r2 = r;
            }
            r1
        }
    }
}

// #![feature(test)]
// extern crate test;
// use test::Bencher;

// #[bench]
// fn bench_workload(b: &mut Bencher) {
//     b.iter(|| workload());
// }

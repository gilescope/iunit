use std::fmt::Debug;
use num_traits::{Num, NumRef}; //NumAssign, NumAssignOps
use trait_tests::trait_tests;

#[trait_tests]
pub trait NumTests : Num + Clone + Debug + NumRef //+ NumAssignOps
{
    fn test_add_sub_zero() {
        assert_eq!((Self::zero() + Self::zero()) - Self::zero(), Self::zero());
        assert_eq!(Self::zero() + (Self::zero() - Self::zero()), Self::zero());
    }

    fn test_add_sub_one() {
        assert!(Self::is_one(&(Self::zero() + Self::one())));

        assert_eq!((Self::zero() + Self::one()) - Self::one(), Self::zero());
    }

    fn test_mul() {
        let multiplied = Self::zero() * Self::zero();
        assert!(Self::is_zero(&multiplied));
        assert_eq!(multiplied, Self::zero());
    }

    fn test_mul_identity() {
        assert_eq!(Self::zero() * Self::one(), Self::zero());
        assert_eq!(Self::one() * Self::one(), Self::one());
    }

    fn test_div_identity() {
        assert_eq!(Self::zero() / Self::one(), Self::zero());
        assert_eq!(Self::one() / Self::one(), Self::one());
    }

    //From num-traits/blob/master/src/lib.rs
    fn check_num_ops() {
        fn compute<T: Num + Clone>(x: T, y: T) -> T {
            let y1 = y.clone();
            let y2 = y.clone();
            let y3 = y.clone();
            let y4 = y.clone();
            x * y / y1 % y2 + y3 - y4
        }
        assert_eq!(compute(Self::one(), Self::one() + Self::one()), Self::one())
    }

    fn check_numref_ops() {
        fn compute<T: NumRef>(x: T, y: &T) -> T {
            x * y / y % y + y - y
        }
        assert_eq!(compute(Self::one(), &(Self::one() + Self::one())), Self::one())
    }

//    fn check_refnum_ops() {
//        fn compute<T: Copy>(x: &T, y: T) -> T
//            where for<'a> &'a T: RefNum<T>
//        {
//            &(&(&(&(x * y) / y) % y) + y) - y
//        }
//        assert_eq!(compute(&(Self::one()), (Self::one() + Self::one())), Self::one())
//    }
//
//    fn check_refref_ops() {
//        fn compute<T>(x: &T, y: &T) -> T
//            where for<'a> &'a T: RefNum<T>
//        {
//            &(&(&(&(x * y) / y) % y) + y) - y
//        }
//        assert_eq!(compute(&(Self::one()), &(Self::one() + Self::one())), Self::one())
//    }
//    // TODO test `NumAssignRef`, but even the standard numeric types don't
//    // implement this yet. (see rust pr41336)

//    fn check_numassign_ops() {
//        fn compute<T: NumAssign + Copy>(mut x: T, y: T) -> T {
//            x *= y;
//            x /= y;
//            x %= y;
//            x += y;
//            x -= y;
//            x
//        }
//        assert_eq!(compute(Self::one(), Self::one() + Self::one()), Self::one())
//    }
}

#[cfg(test)]
mod test {
    use super::*;
    use num_complex::Complex;
    use num_bigint::BigInt;
    use num_rational::BigRational;

    //#[trait_tests] can't be used as usize is not defined in this crate.
    impl NumTests for usize { }
    #[test] fn test_numtests_usize() { <usize as NumTests>::test_all() }

    impl NumTests for u8 { }
    #[test] fn test_numtests_u8() { <usize as NumTests>::test_all() }
    impl NumTests for u16 { }
    #[test] fn test_numtests_u16() { <usize as NumTests>::test_all() }
    impl NumTests for u32 { }
    #[test] fn test_numtests_u32() { <usize as NumTests>::test_all() }
    impl NumTests for u64 { }
    #[test] fn test_numtests_u64() { <usize as NumTests>::test_all() }

    impl NumTests for isize { }
    #[test] fn test_numtests_isize() { <usize as NumTests>::test_all() }
    impl NumTests for i8 { }
    #[test] fn test_numtests_i8() { <usize as NumTests>::test_all() }
    impl NumTests for i16 { }
    #[test] fn test_numtests_i16() { <usize as NumTests>::test_all() }
    impl NumTests for i32 { }
    #[test] fn test_numtests_i32() { <usize as NumTests>::test_all() }
    impl NumTests for i64 { }
    #[test] fn test_numtests_i64() { <usize as NumTests>::test_all() }

    impl NumTests for Complex<f64> { }
    #[test] fn test_numtests_complex_f64() { <usize as NumTests>::test_all() }
    //TODO need to make trait_tests smarter...
    //#[trait_tests] impl NumTests for Complex<i32> { }

    impl NumTests for BigInt { }
    #[test] fn test_numtests_bigint() { <usize as NumTests>::test_all() }
    impl NumTests for BigRational { }
    #[test] fn test_numtests_bigrational() { <usize as NumTests>::test_all() }
}

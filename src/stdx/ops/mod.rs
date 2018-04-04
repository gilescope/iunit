use std::fmt::Debug;
use num::{Num};
use num_traits::{NumRef, NumAssign, NumAssignOps};

#[trait_tests]
pub trait NumTests : Num + Copy + Debug + NumAssignOps + NumRef
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
        fn compute<T: Num + Copy>(x: T, y: T) -> T {
            x * y / y % y + y - y
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

    fn check_numassign_ops() {
        fn compute<T: NumAssign + Copy>(mut x: T, y: T) -> T {
            x *= y;
            x /= y;
            x %= y;
            x += y;
            x -= y;
            x
        }
        assert_eq!(compute(Self::one(), Self::one() + Self::one()), Self::one())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[trait_tests] impl NumTests for usize { }
    #[trait_tests] impl NumTests for u8 { }
    #[trait_tests] impl NumTests for u16 { }
    #[trait_tests] impl NumTests for u32 { }
    #[trait_tests] impl NumTests for u64 { }

    #[trait_tests] impl NumTests for isize { }
    #[trait_tests] impl NumTests for i8 { }
    #[trait_tests] impl NumTests for i16 { }
    #[trait_tests] impl NumTests for i32 { }
    #[trait_tests] impl NumTests for i64 { }
}

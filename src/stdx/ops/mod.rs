use std::fmt::Debug;
use num::Num;

#[trait_tests]
pub trait NumTests : Num + Debug
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
}

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
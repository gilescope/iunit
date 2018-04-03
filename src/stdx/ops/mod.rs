use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Debug;
use num::{Zero, One};

#[trait_tests]
pub trait AddSubTests : Add<Output=Self> + Sub<Output=Self> + Sized + Zero + One + Debug + Eq + Clone
{
    fn test_add_sub_zero() {
        let original = Self::zero();

        let added = original.clone() + Self::zero();

        let result = added - Self::zero();

        assert_eq!(result, original);
    }

    fn test_add_sub_one() {
        let original = Self::zero();

        let added = original.clone() + Self::one();

        assert!(Self::is_one(&added));

        let result = added - Self::one();

        assert_eq!(result, original);
    }
}

#[trait_tests]
pub trait MulDivTests : Mul<Output=Self> + Div<Output=Self> + Sized + Zero + One + Debug + Eq + Clone
{
    fn test_mul() {
        let original = Self::zero();

        let multiplied = original.clone() * Self::zero();
        assert!(Self::is_zero(&multiplied));
        assert_eq!(multiplied, original);
    }
}

#[trait_tests] impl MulDivTests for usize { }

#[trait_tests] impl AddSubTests for usize { }
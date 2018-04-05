use std::fmt::Debug;

#[trait_tests]
pub trait CopyTests: Copy
{
    fn new() -> Self;

    fn test_clone() {
        let me = Self::new();
        let me2 = me; //Copy occurs here.

        assert!(!Self::is_same_address(&me, &me2));
    }

    fn is_same_address(left: &Self, right: &Self) -> bool {
        let a = left as *const _ as *const ();
        let b = right as *const _ as *const ();
        a == b
    }
}


#[trait_tests]
pub trait CloneTests: Clone
{
    fn new() -> Self;

    fn test_clone() {
        let me = Self::new();

        let other1 = me.clone();
        let other2 = me.clone();

        assert!(Self::is_same_address(&me, &me));
        assert!(!Self::is_same_address(&me, &other1));
        assert!(!Self::is_same_address(&me, &other2));
        assert!(!Self::is_same_address(&other1, &other2));
    }

    fn test_clone_a_clone() {
        let me = Self::new();

        let clone1 = me.clone();
        let clone2 = clone1.clone();

        assert!(Self::is_same_address(&me, &me));
        assert!(!Self::is_same_address(&me, &clone1));
        assert!(!Self::is_same_address(&me, &clone2));
        assert!(!Self::is_same_address(&clone1, &clone2));
    }

    fn is_same_address(left: &Self, right: &Self) -> bool {
        let a = left as *const _ as *const ();
        let b = right as *const _ as *const ();
        a == b
    }
}

#[trait_tests]
pub trait EqTests : Eq + Sized {
    fn new() -> Self;

    fn test_eq() {
        let me = &Self::new();

        //Assert Reflective
        assert!(me.eq(me));

        let me_too = &Self::new();

        if me.eq(me_too) {
            //Assert symmetric
            assert!(me_too.eq(me));
        } else {
            assert!(me.ne(me_too));
            assert!(me_too.ne(me));
        }
    }
}

#[trait_tests]
pub trait PartialOrdTests : PartialOrd + Sized {
    fn new() -> Self;

    fn test_partial_ord() {
        let me = &Self::new();
        assert!(me.eq(me));

        let me_too = &Self::new();

        me.ne(me_too);
        //We can't assert that this returns true or false,
        // but we can check it doesn't panic...

        assert!(me.ge(me));
        assert!(me.le(me));
        assert!(!me.gt(me));
        assert!(!me.lt(me));
    }
}

#[trait_tests]
pub trait DebugTests : Debug + Sized {
    fn new() -> Self;

    fn test_new_debug() {
        let me = Self::new();

        format!("{:#?}", &me);

        format!("{:?}", &me);
    }
}

//TODO Error trait, Hash trait

#[cfg(test)]
mod test {
    use super::*;

    #[trait_tests]
    impl CloneTests for String { fn new() -> Self { Self::new() } }

    #[trait_tests]
    impl EqTests for String { fn new() -> Self { Self::new() } }

    #[trait_tests]
    impl DebugTests for String { fn new() -> Self { Self::new() } }

    #[trait_tests]
    impl PartialOrdTests for String { fn new() -> Self { String::new() } }

    #[trait_tests]
    impl CopyTests for i32 { fn new() -> Self { 10 } }

}
use std::fmt::Debug;
use trait_tests::trait_tests;

#[trait_tests]
pub trait CopyTests: Copy + Default
{
    fn test_clone() {
        let me = Self::default();
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
pub trait CloneTests: Clone + Default
{
    fn test_clone() {
        let me = Self::default();

        let other1 = me.clone();
        let other2 = me.clone();

        assert!(Self::is_same_address(&me, &me));
        assert!(!Self::is_same_address(&me, &other1));
        assert!(!Self::is_same_address(&me, &other2));
        assert!(!Self::is_same_address(&other1, &other2));
    }

    fn test_clone_a_clone() {
        let me = Self::default();

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
pub trait EqTests : Eq + Sized + Default {
    fn test_eq() {
        let me = &Self::default();

        //Assert Reflective
        assert!(me.eq(me));

        let me_too = &Self::default();

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
pub trait PartialOrdTests : PartialOrd + Sized + Default {
    fn test_partial_ord() {
        let me = &Self::default();
        assert!(me.eq(me));

        let me_too = &Self::default();

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
pub trait DebugTests : Debug + Sized + Default {
    fn test_new_debug() {
        let me = Self::default();

        format!("{:#?}", &me);

        format!("{:?}", &me);
    }
}

//TODO Error trait, Hash trait

#[cfg(test)]
mod test {
    use super::*;

    //#[trait_tests] can't be used as String struct not defined in this crate.
    impl CloneTests for String {}
    #[test] fn test_clonetests_string() { <String as CloneTests>::test_all() }

    impl EqTests for String {  }
    #[test] fn test_eqtests_string() { <String as EqTests>::test_all() }

    impl DebugTests for String {  }
    #[test] fn test_debugtests_string() { <String as DebugTests>::test_all() }

    impl PartialOrdTests for String {  }
    #[test] fn test_partialordtests_string() { <String as PartialOrdTests>::test_all() }

    impl CopyTests for i32 { }
    #[test] fn test_copytests_string() { <i32 as CopyTests>::test_all() }

}


#[trait_tests]
pub trait CloneTests: Clone
{
    fn new() -> Self;

    fn test_clone() {
        let me = Self::new();

        let other1 = me.clone();
        let other2 = me.clone();

        assert!(Self::is_same(&me, &me));
        assert!(!Self::is_same(&me, &other1));
        assert!(!Self::is_same(&me, &other2));
        assert!(!Self::is_same(&other1, &other2));
    }

    fn test_clone_a_clone() {
        let me = Self::new();

        let clone1 = me.clone();
        let clone2 = clone1.clone();

        assert!(Self::is_same(&me, &me));
        assert!(!Self::is_same(&me, &clone1));
        assert!(!Self::is_same(&me, &clone2));
        assert!(!Self::is_same(&clone1, &clone2));
    }

    fn is_same(left: &Self, right: &Self) -> bool {
        let a = left as *const _ as *const ();
        let b = right as *const _ as *const ();
        a == b
    }
}

#[trait_tests]
impl CloneTests for String {
    fn new() -> Self { String::new() }
}
use std::fmt::Debug;
//use std::iter::FromIterator;
use std::hash as hash;
use std::collections::{HashSet, BTreeSet};
use super::CollectionTests;

use eclectic::{Set, AddRemove};

#[derive(Debug)]
pub struct Foo(&'static str, i32);

impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Foo {
    fn partial_cmp(&self, other: &Foo) -> Option<::std::cmp::Ordering> {
        self.0.partial_cmp(other.0)
    }
}

impl Ord for Foo {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.0.cmp(other.0)
    }
}

impl Eq for Foo {}

impl hash::Hash for Foo {
    fn hash<H: hash::Hasher>(&self, h: &mut H) {
        self.0.hash(h);
    }
}

//TODO no drain or retain on BTreeSet. Leave out those tests for now.

#[trait_tests]
pub trait SetTestsisize: Set<Item=isize> + CollectionTests
//+ FromIterator<isize>
//+ IntoIterator<Item=isize>
+ Debug
+ Eq
+ Sized
+ AddRemove
+ Default
{
    // This is sub-optimal but currently #[test] excludes all generics.is_parameterized()
    // despite their being no unfilled parameters. (src/libsyntax/test.rs)
    // We are autogenerating a test_all() function using a compiler plugin: #[trait_tests]

//    fn test_super() {
//        CollectionTests::test_all();
//    }

    fn test_disjoint()
    {
        let mut xs = Self::default();
        let mut ys = Self::default();

        assert!(xs.is_disjoint(&ys));
        assert!(ys.is_disjoint(&xs));
        assert!(xs.insert(5isize));
        assert!(ys.insert(11));
        assert!(xs.is_disjoint(&ys));
        assert!(ys.is_disjoint(&xs));
        assert!(xs.insert(7));
        assert!(xs.insert(19));
        assert!(xs.insert(4));
        assert!(ys.insert(2));
        assert!(ys.insert(-11));
        assert!(xs.is_disjoint(&ys));
        assert!(ys.is_disjoint(&xs));
        assert!(ys.insert(7));
        assert!(!xs.is_disjoint(&ys));
        assert!(!ys.is_disjoint(&xs));
    }

    fn test_subset_and_superset()
    {
        let mut a = Self::default();
        assert!(a.insert(0));
        assert!(a.insert(5));
        assert!(a.insert(11));
        assert!(a.insert(7));

        let mut b = Self::default();
        assert!(b.insert(0));
        assert!(b.insert(7));
        assert!(b.insert(19));
        assert!(b.insert(250));
        assert!(b.insert(11));
        assert!(b.insert(200));

        assert!(!a.is_subset(&b));
        assert!(!a.is_superset(&b));
        assert!(!b.is_subset(&a));
        assert!(!b.is_superset(&a));

        assert!(b.insert(5));

        assert!(a.is_subset(&b));
        assert!(!a.is_superset(&b));
        assert!(!b.is_subset(&a));
        assert!(b.is_superset(&a));
    }

    fn test_iterate()
    {
        let mut a = Self::default();
        for i in 0..32 {
            assert!(a.insert(i));
        }
        let mut observed: u32 = 0;
        for k in a.iter() {
            observed |= 1 << *k as u32;
        }
        assert_eq!(observed, 0xFFFF_FFFF);
    }

//    fn test_intersection()
//    {
//        let mut a = Self::new();
//        let mut b = Self::new();
//
//        assert!(a.insert(11));
//        assert!(a.insert(1));
//        assert!(a.insert(3));
//        assert!(a.insert(77));
//        assert!(a.insert(103));
//        assert!(a.insert(5));
//        assert!(a.insert(-5));
//
//        assert!(b.insert(2));
//        assert!(b.insert(11));
//        assert!(b.insert(77));
//        assert!(b.insert(-9));
//        assert!(b.insert(-42));
//        assert!(b.insert(5));
//        assert!(b.insert(3));
//
//        let mut i = 0;
//        let expected = [3, 5, 11, 77];
//        for x in a.intersection(&b) {
//            assert!(expected.contains(&x));
//            i += 1
//        }
//        assert_eq!(i, expected.len());
//    }

//    fn test_difference()
//    {
//        let mut a = Self::new();
//        let mut b = Self::new();
//
//        assert!(a.insert(1));
//        assert!(a.insert(3));
//        assert!(a.insert(5));
//        assert!(a.insert(9));
//        assert!(a.insert(11));
//
//        assert!(b.insert(3));
//        assert!(b.insert(9));
//
//        let mut i = 0;
//        let expected = [1, 5, 11];
//        for x in a.difference(&b) {
//            assert!(expected.contains(x));
//            i += 1
//        }
//        assert_eq!(i, expected.len());
//    }

//    fn test_symmetric_difference()
//    {
//        let mut a = Self::new();
//        let mut b = Self::new();
//
//        assert!(a.insert(1));
//        assert!(a.insert(3));
//        assert!(a.insert(5));
//        assert!(a.insert(9));
//        assert!(a.insert(11));
//
//        assert!(b.insert(-2));
//        assert!(b.insert(3));
//        assert!(b.insert(9));
//        assert!(b.insert(14));
//        assert!(b.insert(22));
//
//        let mut i = 0;
//        let expected = [-2, 1, 5, 11, 14, 22];
//        for x in a.symmetric_difference(&b) {
//            assert!(expected.contains(x));
//            i += 1
//        }
//        assert_eq!(i, expected.len());
//    }
//
//    fn test_union()
//    {
//        let mut a = Self::new();
//        let mut b = Self::new();
//
//        assert!(a.insert(1));
//        assert!(a.insert(3));
//        assert!(a.insert(5));
//        assert!(a.insert(9));
//        assert!(a.insert(11));
//        assert!(a.insert(16));
//        assert!(a.insert(19));
//        assert!(a.insert(24));
//
//        assert!(b.insert(-2));
//        assert!(b.insert(1));
//        assert!(b.insert(5));
//        assert!(b.insert(9));
//        assert!(b.insert(13));
//        assert!(b.insert(19));
//
//        let mut i = 0;
//        let expected = [-2, 1, 3, 5, 9, 11, 13, 16, 19, 24];
//        for x in a.union(&b) {
//            assert!(expected.contains(x));
//            i += 1
//        }
//        assert_eq!(i, expected.len());
//    }

//    fn test_from_iter()
//    {
//        let xs = [1, 2, 3, 4, 5, 6, 7, 8, 9];
//
//        let set: Self = xs.iter().cloned().collect();
//
//        for x in &xs {
//            assert!(set.contains(x));
//        }
//    }

    fn test_eq()
    {
        // These constants once happened to expose a bug in insert().
        // I'm keeping them around to prevent a regression.
        let mut s1 = Self::default();

        s1.insert(1);
        s1.insert(2);
        s1.insert(3);

        let mut s2 = Self::default();

        s2.insert(1);
        s2.insert(2);

        assert!(s1 != s2);

        s2.insert(3);

        assert_eq!(s1, s2);
    }

    fn test_show()
    {
        let mut set = Self::default();
        let empty = Self::default();

        set.insert(1);
        set.insert(2);

        let set_str = format!("{:?}", set);

        assert!(set_str == "{1, 2}" || set_str == "{2, 1}");
        assert_eq!(format!("{:?}", empty), "{}");
    }

//    fn test_extend_ref()
//    {
//        let mut a = Self::new();
//        a.insert(1);
//
//        a.extend(vec![2isize, 3, 4]); //TODO should be &[2,3,4]
//
//        assert_eq!(a.len(), 4);
//        assert!(a.contains(&1));
//        assert!(a.contains(&2));
//        assert!(a.contains(&3));
//        assert!(a.contains(&4));
//
//        let mut b = Self::new();
//        b.insert(5);
//        b.insert(6);
//
//        a.extend(b);
//
//        assert_eq!(a.len(), 6);
//        assert!(a.contains(&1));
//        assert!(a.contains(&2));
//        assert!(a.contains(&3));
//        assert!(a.contains(&4));
//        assert!(a.contains(&5));
//        assert!(a.contains(&6));
//    }
}

#[trait_tests]
pub trait SetTestsfoo: Set<Item=Foo> + Sized + IntoIterator<Item=Foo> + AddRemove + Default
{
    fn test_replace()
    {
        let mut s = Self::default();
        assert_eq!(s.replace(Foo("a", 1)), None);
        assert_eq!(s.len(), 1);
        assert_eq!(s.replace(Foo("a", 2)), Some(Foo("a", 1)));
        assert_eq!(s.len(), 1);

        let mut it = s.iter();
        assert_eq!(it.next(), Some(&Foo("a", 2)));
        assert_eq!(it.next(), None);
    }
}

#[trait_tests]
pub trait SetTestschar: Set<Item=char> + Sized + IntoIterator<Item=char> + AddRemove + Default
{
    //#[test]
    fn test_move_iter()
    {
        let hs = {
            let mut hs = Self::default();

            hs.insert('a');
            hs.insert('b');

            hs
        };

        let v = hs.into_iter().collect::<Vec<char>>();
        assert!(v == ['a', 'b'] || v == ['b', 'a']);
    }
}

//Have to either be here or in the std crate:
#[trait_tests] impl SetTestsisize for HashSet<isize> { }
#[trait_tests] impl CollectionTests for HashSet<isize> { fn new() -> Self { Self::new() } }

#[trait_tests] impl SetTestsfoo for HashSet<Foo> {  }
#[trait_tests] impl SetTestschar for HashSet<char> { }

//Have to either be here or in the std crate:
#[trait_tests] impl SetTestsisize for BTreeSet<isize> { }
#[trait_tests] impl CollectionTests for BTreeSet<isize> { fn new() -> Self { Self::new() }}


#[trait_tests] impl SetTestsfoo for BTreeSet<Foo> { }
#[trait_tests] impl SetTestschar for BTreeSet<char> { }
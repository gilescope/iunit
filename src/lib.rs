//!
//! goals:
//!
//! [x] Trait linked to TraitTests
//!
//! [ ] Impl annotation creates tests.
//!
//! additional:
//!
//! [ ] Tests covering multiple Traits
//!
#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(test_trait_derive)]

//#[macro_use] extern crate test_trait_derive;

mod stdx {
    mod collections {
        use ::std::collections::btree_set::BTreeSet;
        use ::std::collections::HashSet;
        use ::std::hash::Hash;
        use ::std::iter::IntoIterator;

        pub trait Set<T> where T: Eq + Hash
        {
            fn new() -> Self;

            fn insert(&mut self, item: T) -> bool;

            fn is_disjoint(&self, other: &Self) -> bool;

            fn is_subset(&self, other: &Self) -> bool;

            fn is_superset(&self, other: &Self) -> bool;

            fn intersection<'a>(&'a self, other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a>;

            fn contains(&self, item: &T) -> bool;

            fn difference<'a>(&'a self, other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a>;

            fn symmetric_difference<'a>(&'a self, other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a>;

            fn union<'a>(&'a self, other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a>;

            fn is_empty(&self) -> bool;

            fn len(&self) -> usize;

            fn replace(&mut self, value: T) -> Option<T>;

            fn extend<Iter>(&mut self, iter: Iter)
                where Iter: IntoIterator<Item=T>;

            fn iter<'a>(&'a self) -> Box<Iterator<Item=&'a T> + 'a>;
        }

        impl<T> Set<T> for HashSet<T> where T: Eq + Hash
        {
            fn new() -> HashSet<T> { HashSet::new() }

            fn insert(&mut self, item: T) -> bool { self.insert(item) }

            fn is_disjoint(&self, other: &Self) -> bool { self.is_disjoint(other) }

            fn is_subset(&self, other: &Self) -> bool { self.is_subset(other) }

            fn is_superset(&self, other: &Self) -> bool { self.is_superset(other) }

            fn intersection<'a>(&'a self, other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a>
            {
                Box::new(self.intersection(other))
            }

            fn contains(&self, item: &T) -> bool { self.contains(item) }

            fn difference<'a>(&'a self, other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a>
            {
                Box::new(self.difference(other))
            }

            fn symmetric_difference<'a>(&'a self, other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a>
            {
                Box::new(self.symmetric_difference(other))
            }

            fn union<'a>(&'a self, other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a>
            {
                Box::new(self.union(other))
            }

            fn is_empty(&self) -> bool { self.is_empty() }

            fn len(&self) -> usize { self.len() }

            fn replace(&mut self, value: T) -> Option<T> { self.replace(value) }

            fn extend<Iter: IntoIterator<Item=T>>(&mut self, iter: Iter) {
                Extend::extend(self, iter);
            }

            fn iter<'a>(&'a self) -> Box<Iterator<Item=&'a T> + 'a>
            {
                Box::new(self.iter())
            }
        }

        impl<T> Set<T> for BTreeSet<T> where T: Eq + Hash + Ord
        {
            fn new() -> BTreeSet<T> { BTreeSet::new() }

            fn insert(&mut self, item: T) -> bool { self.insert(item) }

            fn is_disjoint(&self, other: &Self) -> bool { self.is_disjoint(other) }

            fn is_subset(&self, other: &Self) -> bool { self.is_subset(other) }

            fn is_superset(&self, other: &Self) -> bool { self.is_superset(other) }

            fn intersection<'a>(&'a self, other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a>
            {
                Box::new(self.intersection(other))
            }

            fn contains(&self, item: &T) -> bool { self.contains(item) }

            fn difference<'a>(&'a self, other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a>
            {
                Box::new(self.difference(other))
            }

            fn symmetric_difference<'a>(&'a self, other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a>
            {
                Box::new(self.symmetric_difference(other))
            }

            fn union<'a>(&'a self, other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a>
            {
                Box::new(self.union(other))
            }

            fn is_empty(&self) -> bool { self.is_empty() }

            fn len(&self) -> usize { self.len() }

            fn replace(&mut self, value: T) -> Option<T> { self.replace(value) }

            fn extend<Iter: IntoIterator<Item=T>>(&mut self, iter: Iter) {
                Extend::extend(self, iter);
            }

            fn iter<'a>(&'a self) -> Box<Iterator<Item=&'a T> + 'a>
            {
                Box::new(self.iter())
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use ::std::fmt::Debug;
            use ::std::iter::FromIterator;
            use ::std::hash as hash;

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

            #[trait_tests]
            pub trait SetTestsisize: Set<isize>
                + FromIterator<isize> + IntoIterator<Item=isize>
                + Debug + Eq + Sized
            {
                // This is sub-optimal but currently #[test] excludes all generics.is_parameterized()
                // despite their being no unfilled parameters. (src/libsyntax/test.rs)
                // We are autogenerating this function using a compiler plugin: #[trait_tests]
//                fn test_all() {
//                    Self::test_disjoint();
//                    Self::test_subset_and_superset();
//                    Self::test_iterate();
//                    Self::test_intersection();
//                    Self::test_difference();
//                    Self::test_symmetric_difference();
//                    Self::test_union();
//                    Self::test_from_iter();
//                    Self::test_eq();
//                    Self::test_show();
//                    Self::test_extend_ref();
//                }

                #[test]
                fn test_disjoint()
                {
                    println!("test disjoint running");
                    let mut xs = Self::new();
                    let mut ys = Self::new();
                    assert!(xs.is_disjoint(&ys));
                    assert!(ys.is_disjoint(&xs));
                    assert!(xs.insert(5));
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

                #[test]
                fn test_subset_and_superset()
                {
                    println!("test subset running");

                    let mut a = Self::new();
                    assert!(a.insert(0));
                    assert!(a.insert(5));
                    assert!(a.insert(11));
                    assert!(a.insert(7));

                    let mut b = Self::new();
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

                #[test]
                fn test_iterate()
                {
                    let mut a = Self::new();
                    for i in 0..32 {
                        assert!(a.insert(i));
                    }
                    let mut observed: u32 = 0;
                    for k in a {
                        observed |= 1 << k;
                    }
                    assert_eq!(observed, 0xFFFF_FFFF);
                }

                #[test]
                fn test_intersection()
                {
                    let mut a = Self::new();
                    let mut b = Self::new();

                    assert!(a.insert(11));
                    assert!(a.insert(1));
                    assert!(a.insert(3));
                    assert!(a.insert(77));
                    assert!(a.insert(103));
                    assert!(a.insert(5));
                    assert!(a.insert(-5));

                    assert!(b.insert(2));
                    assert!(b.insert(11));
                    assert!(b.insert(77));
                    assert!(b.insert(-9));
                    assert!(b.insert(-42));
                    assert!(b.insert(5));
                    assert!(b.insert(3));

                    let mut i = 0;
                    let expected = [3, 5, 11, 77];
                    for x in a.intersection(&b) {
                        assert!(expected.contains(&x));
                        i += 1
                    }
                    assert_eq!(i, expected.len());
                }

                #[test]
                fn test_difference()
                {
                    let mut a = Self::new();
                    let mut b = Self::new();

                    assert!(a.insert(1));
                    assert!(a.insert(3));
                    assert!(a.insert(5));
                    assert!(a.insert(9));
                    assert!(a.insert(11));

                    assert!(b.insert(3));
                    assert!(b.insert(9));

                    let mut i = 0;
                    let expected = [1, 5, 11];
                    for x in a.difference(&b) {
                        assert!(expected.contains(x));
                        i += 1
                    }
                    assert_eq!(i, expected.len());
                }

                #[test]
                fn test_symmetric_difference()
                {
                    let mut a = Self::new();
                    let mut b = Self::new();

                    assert!(a.insert(1));
                    assert!(a.insert(3));
                    assert!(a.insert(5));
                    assert!(a.insert(9));
                    assert!(a.insert(11));

                    assert!(b.insert(-2));
                    assert!(b.insert(3));
                    assert!(b.insert(9));
                    assert!(b.insert(14));
                    assert!(b.insert(22));

                    let mut i = 0;
                    let expected = [-2, 1, 5, 11, 14, 22];
                    for x in a.symmetric_difference(&b) {
                        assert!(expected.contains(x));
                        i += 1
                    }
                    assert_eq!(i, expected.len());
                }

                #[test]
                fn test_union()
                {
                    let mut a = Self::new();
                    let mut b = Self::new();

                    assert!(a.insert(1));
                    assert!(a.insert(3));
                    assert!(a.insert(5));
                    assert!(a.insert(9));
                    assert!(a.insert(11));
                    assert!(a.insert(16));
                    assert!(a.insert(19));
                    assert!(a.insert(24));

                    assert!(b.insert(-2));
                    assert!(b.insert(1));
                    assert!(b.insert(5));
                    assert!(b.insert(9));
                    assert!(b.insert(13));
                    assert!(b.insert(19));

                    let mut i = 0;
                    let expected = [-2, 1, 3, 5, 9, 11, 13, 16, 19, 24];
                    for x in a.union(&b) {
                        assert!(expected.contains(x));
                        i += 1
                    }
                    assert_eq!(i, expected.len());
                }

                #[test]
                fn test_from_iter()
                {
                    let xs = [1, 2, 3, 4, 5, 6, 7, 8, 9];

                    let set: Self = xs.iter().cloned().collect();

                    for x in &xs {
                        assert!(set.contains(x));
                    }
                }

                #[test]
                fn test_eq()
                {
                    // These constants once happened to expose a bug in insert().
                    // I'm keeping them around to prevent a regression.
                    let mut s1 = Self::new();

                    s1.insert(1);
                    s1.insert(2);
                    s1.insert(3);

                    let mut s2 = Self::new();

                    s2.insert(1);
                    s2.insert(2);

                    assert!(s1 != s2);

                    s2.insert(3);

                    assert_eq!(s1, s2);
                }

                #[test]
                fn test_show()
                {
                    let mut set = Self::new();
                    let empty = Self::new();

                    set.insert(1);
                    set.insert(2);

                    let set_str = format!("{:?}", set);

                    assert!(set_str == "{1, 2}" || set_str == "{2, 1}");
                    assert_eq!(format!("{:?}", empty), "{}");
                }

                #[test]
                fn test_extend_ref()
                {
                    let mut a = Self::new();
                    a.insert(1);

                    a.extend(vec![2isize, 3, 4]); //TODO should be &[2,3,4]

                    assert_eq!(a.len(), 4);
                    assert!(a.contains(&1));
                    assert!(a.contains(&2));
                    assert!(a.contains(&3));
                    assert!(a.contains(&4));

                    let mut b = Self::new();
                    b.insert(5);
                    b.insert(6);

                    a.extend(b);

                    assert_eq!(a.len(), 6);
                    assert!(a.contains(&1));
                    assert!(a.contains(&2));
                    assert!(a.contains(&3));
                    assert!(a.contains(&4));
                    assert!(a.contains(&5));
                    assert!(a.contains(&6));
                }
            }

            #[trait_tests]
            pub trait SetTestsfoo: Set<Foo> + Sized + IntoIterator<Item=Foo>
            {
                #[test]
                fn test_replace()
                {
                    let mut s = Self::new();
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
            pub trait SetTestschar: Set<char> + Sized + IntoIterator<Item=char>
            {
                #[test]
                fn test_move_iter()
                {
                    let hs = {
                        let mut hs = Self::new();

                        hs.insert('a');
                        hs.insert('b');

                        hs
                    };

                    let v = hs.into_iter().collect::<Vec<char>>();
                    assert!(v == ['a', 'b'] || v == ['b', 'a']);
                }
            }
        }

        /// TODO: the code in impl_tests could be autogenerated with a macro
        #[cfg(test)]
        mod impl_tests {
            use super::tests::*;
            use super::*;

            macro_rules! trait_test {
                ($impl_name:ty, $test_name:ty, $test_type:ty, $id:ident, $test_path:path) => (
                        impl $test_name for $impl_name {}
                        #[test] fn $id() { $test_path(); }
                )
            }

            trait_test!(HashSet<isize>, SetTestsisize, isize, test1, HashSet::<isize>::test_all);
            trait_test!(HashSet<char>, SetTestschar,  char, test2, HashSet::<char>::test_all);
            trait_test!(HashSet<Foo>, SetTestsfoo,  Foo, test3, HashSet::<Foo>::test_all);

            trait_test!(BTreeSet<isize>, SetTestsisize, isize, test4, BTreeSet::<isize>::test_all);
            trait_test!(BTreeSet<char>, SetTestschar,  char, test5, BTreeSet::<char>::test_all);
            trait_test!(BTreeSet<Foo>, SetTestsfoo,  Foo, test6, BTreeSet::<Foo>::test_all);

            //Derive macro
            //impl SetTestsisize for HashSet<isize> {}
//            impl SetTestschar for HashSet<char> {}
//            impl SetTestsfoo for HashSet<Foo> {}

            //#[derive(TraitTests)]
            //#[ForType(isize)]
            //impl SetTestsisize for HashSet<isize> {}
            //#[test] fn HashSet_tests1() { HashSet::<isize>::test_all(); }

            //#[test] fn HashSet_tests2() { HashSet::<char>::test_all(); }
            //#[test] fn HashSet_tests3() { HashSet::<Foo>::test_all(); }

//            impl SetTestsisize for BTreeSet<isize> {}
//            impl SetTestschar for BTreeSet<char> {}
//            impl SetTestsfoo for BTreeSet<Foo> {}
//
//            #[test] fn BTreeSet_tests1() { BTreeSet::<isize>::test_all(); }
//            #[test] fn BTreeSet_tests2() { BTreeSet::<char>::test_all(); }
//            #[test] fn BTreeSet_tests3() { BTreeSet::<Foo>::test_all(); }
        }
    }
}

//TODO no drain or retain on BTreeSet. Leave out those tests for now.
//Oct12th.Eng
#![feature(custom_attribute)]

use syntax::parse::token;
use syntax::tokenstream::TokenTree;
use syntax::ext::base::ExtCtxt;
use rustc_plugin::Registry;

mod stdx {
    mod collections {
        use ::std::collections::btree_set::BTreeSet;
        use ::std::collections::HashSet;
        use ::std::hash::Hash;

        pub trait Set<T> where T: Eq + Hash
        {
            fn new() -> Self;

            fn insert(&mut self, item:T) -> bool;

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
                where Iter : IntoIterator<Item = T>;

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

            fn extend<Iter: IntoIterator<Item = T>>(&mut self, iter: Iter) {
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

            fn extend<Iter: IntoIterator<Item = T>>(&mut self, iter: Iter) {
                Extend::extend(self, iter);
            }

            fn iter<'a>(&'a self) -> Box<Iterator<Item=&'a T> + 'a>
            {
                Box::new(self.iter())
            }
        }

        #[cfg(test)]
        mod tests {
            use std::collections::HashSet;
            use std::collections::BTreeSet;
            use std::fmt::Debug;

            #[test]
            fn test_set_hash() {
                test_set_trait::<HashSet<_>>();
                test_move_iter::<HashSet<_>>();
                test_replace::<HashSet<_>>();
            }

            #[test]
            fn test_set_btree() {
                test_set_trait::<BTreeSet<_>>();
                test_move_iter::<BTreeSet<_>>();
                test_replace::<BTreeSet<_>>();
            }

            fn test_set_trait<S>()
                where S: super::Set<isize>
                + IntoIterator<Item=isize>
                + ::std::iter::FromIterator<isize>
                + Eq + Debug
            {
                test_disjoint::<S>();
                test_subset_and_superset::<S>();
                test_iterate::<S>();
                test_intersection::<S>();

                test_from_iter::<S>();
                test_symmetric_difference::<S>();
                test_difference::<S>();
                test_union::<S>();

                test_eq::<S>();
                test_show::<S>();
                test_extend_ref::<S>();

            }

            macro_rules! test_traits {
                // `()` indicates that the macro takes no argument.
                () => (
                    // The macro will expand into the contents of this block.
                    #[test]
                    fn test_tra() {
                        println!("Hi");
                    }
                )
            }

            // The following are Rust's HashSet tests with minimal modifications.

            #[test_traits]
            fn test_disjoint<S>()
                where S: super::Set<isize>
            {
                let mut xs = S::new();
                let mut ys = S::new();
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

            fn test_subset_and_superset<S>()
                where S: super::Set<isize>
            {
                let mut a = S::new();
                assert!(a.insert(0));
                assert!(a.insert(5));
                assert!(a.insert(11));
                assert!(a.insert(7));

                let mut b = S::new();
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

            fn test_iterate<S>()
                where S: super::Set<isize>
                + IntoIterator<Item=isize>
            {
                let mut a = S::new();
                for i in 0..32 {
                    assert!(a.insert(i));
                }
                let mut observed: u32 = 0;
                for k in a {
                    observed |= 1 << k;
                }
                assert_eq!(observed, 0xFFFF_FFFF);
            }

            fn test_intersection<S>()
                where S: super::Set<isize>
                + IntoIterator<Item=isize>
            {
                let mut a = S::new();
                let mut b = S::new();

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

            fn test_difference<S>()
                where S: super::Set<isize>
                + IntoIterator<Item=isize>
            {
                let mut a = S::new();
                let mut b = S::new();

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

            fn test_symmetric_difference<S>()
                where S: super::Set<isize>
                + IntoIterator<Item=isize>
            {
                let mut a = S::new();
                let mut b = S::new();

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

            fn test_union<S>()
                where S: super::Set<isize>
                + IntoIterator<Item=isize>
            {
                let mut a = S::new();
                let mut b = S::new();

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


            fn test_from_iter<S>()
                where S: ::std::iter::FromIterator<isize> + super::Set<isize>
            {
                let xs = [1, 2, 3, 4, 5, 6, 7, 8, 9];

                let set: S = xs.iter().cloned().collect();

                for x in &xs {
                    assert!(set.contains(x));
                }
            }

            fn test_move_iter<S>()
                where S: super::Set<char>
                + IntoIterator<Item=char>
            {
                let hs = {
                    let mut hs = S::new();

                    hs.insert('a');
                    hs.insert('b');

                    hs
                };

                let v = hs.into_iter().collect::<Vec<char>>();
                assert!(v == ['a', 'b'] || v == ['b', 'a']);
            }

            fn test_eq<S>()
                where S: super::Set<isize> + Eq + Debug
            {
                // These constants once happened to expose a bug in insert().
                // I'm keeping them around to prevent a regression.
                let mut s1 = S::new();

                s1.insert(1);
                s1.insert(2);
                s1.insert(3);

                let mut s2 = S::new();

                s2.insert(1);
                s2.insert(2);

                assert!(s1 != s2);

                s2.insert(3);

                assert_eq!(s1, s2);
            }


            fn test_show<S>()
                where S: super::Set<isize> + Eq + Debug
            {
                let mut set = S::new();
                let empty = S::new();

                set.insert(1);
                set.insert(2);

                let set_str = format!("{:?}", set);

                assert!(set_str == "{1, 2}" || set_str == "{2, 1}");
                assert_eq!(format!("{:?}", empty), "{}");
            }

//TODO no drain on BTreeSet. Leave off Trait for now.
//            fn test_trivial_drain<S>()
//                where S: super::Set<isize> {
//                let mut s = S::new();
//                for _ in s.drain() {}
//                assert!(s.is_empty());
//                drop(s);
//
//                let mut s = S::new();
//                drop(s.drain());
//                assert!(s.is_empty());
//            }
//
//            fn test_drain<S>()
//                where S: super::Set<isize> {
//                let mut s: S = (1..100).collect();
//
//                // try this a bunch of times to make sure we don't screw up internal state.
//                for _ in 0..20 {
//                    assert_eq!(s.len(), 99);
//
//                    {
//                        let mut last_i = 0;
//                        let mut d = s.drain();
//                        for (i, x) in d.by_ref().take(50).enumerate() {
//                            last_i = i;
//                            assert!(x != 0);
//                        }
//                        assert_eq!(last_i, 49);
//                    }
//
//                    for _ in &s {
//                        panic!("s should be empty!");
//                    }
//
//                    // reset to try again.
//                    s.extend(1..100);
//                }
//            }

            use ::std::hash as hash;
            #[derive(Debug)]
            struct Foo(&'static str, i32);

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

            fn test_replace<S>() //TODO fails for BTreeSet
                where S: super::Set<Foo>
                + IntoIterator<Item=Foo> {

                let mut s = S::new();
                assert_eq!(s.replace(Foo("a", 1)), None);
                assert_eq!(s.len(), 1);
                assert_eq!(s.replace(Foo("a", 2)), Some(Foo("a", 1)));
                assert_eq!(s.len(), 1);

                let mut it = s.iter();
                assert_eq!(it.next(), Some(&Foo("a", 2)));
                assert_eq!(it.next(), None);
            }

            fn test_extend_ref<S>()
                where S: super::Set<isize>
                + IntoIterator<Item=isize>
            {
                let mut a = S::new();
                a.insert(1);

                a.extend(vec![2isize, 3, 4] ); //TODO should be &[2,3,4]

                assert_eq!(a.len(), 4);
                assert!(a.contains(&1));
                assert!(a.contains(&2));
                assert!(a.contains(&3));
                assert!(a.contains(&4));

                let mut b = S::new();
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

//Not on BTreeSet
//            fn test_retain<S>() where S: super::Set<isize> {
//                let xs = [1, 2, 3, 4, 5, 6];
//                let mut set: S = xs.iter().cloned().collect();
//                set.retain(|&k| k % 2 == 0);
//                assert_eq!(set.len(), 3);
//                assert!(set.contains(&2));
//                assert!(set.contains(&4));
//                assert!(set.contains(&6));
//            }
        }
    }
}
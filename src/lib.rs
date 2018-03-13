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
        }

        impl<T> Set<T> for BTreeSet<T> where T: Eq + Hash + Ord
        {
            fn new() -> BTreeSet<T> { BTreeSet::new() }

            fn insert(&mut self, item: T) -> bool { self.insert(item) }

            fn is_disjoint(&self, other: &Self) -> bool { self.is_disjoint(other) }

            fn is_subset(&self, other: &Self) -> bool { self.is_subset(other) }

            fn is_superset(&self, other: &Self) -> bool { self.is_superset(other) }

            fn intersection<'b>(&'b self, other: &'b Self) -> Box<Iterator<Item=&'b T> + 'b>
            {
                Box::new(self.intersection(other))
            }

            fn contains(&self, item: &T) -> bool { self.contains(item) }
        }

        #[cfg(test)]
        mod tests {
            use std::collections::HashSet;
            use std::collections::BTreeSet;

            #[test]
            fn test_set_hash() {
                test_set_trait::<HashSet<isize>>();
            }

            #[test]
            fn test_set_btree() {
                test_set_trait::<BTreeSet<isize>>();
            }

            fn test_set_trait<S>()
                where S: super::Set<isize>
                + IntoIterator<Item=isize>
                + ::std::iter::FromIterator<isize>
            {
                test_disjoint_iunit::<S>();
                test_subset_and_superset_iunit::<S>();
                test_iterate::<S>();
                test_intersection::<S>();
                test_from_iter::<S>();
            }

            fn test_disjoint_iunit<S>()
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

            fn test_subset_and_superset_iunit<S>()
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

            fn test_from_iter<S>()
                where S: ::std::iter::FromIterator<isize> + super::Set<isize>
            {
                let xs = [1, 2, 3, 4, 5, 6, 7, 8, 9];

                let set: S = xs.iter().cloned().collect();

                for x in &xs {
                    assert!(set.contains(x));
                }
            }

//            fn test_move_iter<S>() {
//                let hs = {
//                    let mut hs = HashSet::new();
//
//                    hs.insert('a');
//                    hs.insert('b');
//
//                    hs
//                };
//
//                let v = hs.into_iter().collect::<Vec<char>>();
//                assert!(v == ['a', 'b'] || v == ['b', 'a']);
//            }

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
        }
    }
}
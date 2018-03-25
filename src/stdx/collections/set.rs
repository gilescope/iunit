
        use std::collections::btree_set::BTreeSet;
        use std::collections::HashSet;
        use std::hash::Hash;
        use std::iter::IntoIterator;

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

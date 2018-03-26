#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(trait_tests)]

extern crate iunit;

#[cfg(test)]
mod impl_tests {
    use iunit::stdx::collections::Set;
    //use iunit::stdx::collections::tests::{SetTestsisize, SetTestsfoo, SetTestschar, Foo};
    use std::hash::Hash;

    //MySet example of minimal implementation to run the tests.
    #[derive(Debug,Eq,PartialEq)]
    struct MySet<T> {
        store: Vec<T>,
    }

    impl <I> ::std::iter::FromIterator<I> for MySet<I> {
        fn from_iter<T: IntoIterator<Item=I>>(_iter: T) -> Self {
            //<::stdx::collections::impl_tests::MySet::<isize> as SetTestsisize>::test_all();
            unimplemented!()
        }
    }

    impl <T> Set<T> for MySet<T>
        where T : Hash + Eq
    {
        fn new() -> Self {
            unimplemented!()
        }

        fn insert(&mut self, _item: T) -> bool {
            unimplemented!()
        }

        fn is_disjoint(&self, _other: &Self) -> bool {
            unimplemented!()
        }

        fn is_subset(&self, _other: &Self) -> bool {
            unimplemented!()
        }

        fn is_superset(&self, _other: &Self) -> bool {
            unimplemented!()
        }

        fn intersection<'a>(&'a self, _other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a> {
            unimplemented!()
        }

        fn contains(&self, _item: &T) -> bool {
            unimplemented!()
        }

        fn difference<'a>(&'a self, _other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a> {
            unimplemented!()
        }

        fn symmetric_difference<'a>(&'a self, _other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a> {
            unimplemented!()
        }

        fn union<'a>(&'a self, _other: &'a Self) -> Box<Iterator<Item=&'a T> + 'a> {
            unimplemented!()
        }

        fn is_empty(&self) -> bool {
            unimplemented!()
        }

        fn len(&self) -> usize {
            unimplemented!()
        }

        fn replace(&mut self, _value: T) -> Option<T> {
            unimplemented!()
        }

        fn extend<Iter>(&mut self, _iter: Iter) where Iter: IntoIterator<Item=T> {
            unimplemented!()
        }

        fn iter<'a>(&'a self) -> Box<Iterator<Item=&'a T> + 'a> {
            unimplemented!()
        }
    }

    impl <T> Iterator for MySet<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            unimplemented!()
        }
    }

//    #[trait_tests] impl SetTestsisize for MySet<isize> {}
//    #[trait_tests] impl SetTestsfoo for MySet<Foo> {}
//    #[trait_tests] impl SetTestschar for MySet<char> {}
}
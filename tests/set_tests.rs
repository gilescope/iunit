//#![feature(custom_attribute)]
//#![feature(plugin)]
//#![plugin(trait_tests)]
//
//extern crate iunit;
//
//
//extern crate eclectic;
//
//#[cfg(test)]
//mod impl_tests {
////    use eclectic::{Set, AddRemove};
//
//    //use iunit::stdx::collections::tests::{SetTestsisize, SetTestsfoo, SetTestschar, Foo};
////    use std::hash::Hash;
////
////    //MySet example of minimal implementation to run the tests.
////    #[derive(Debug,Eq,PartialEq)]
////    struct MySet<T> {
////        store: Vec<T>,
////    }
////
////    impl <I> ::std::iter::FromIterator<I> for MySet<I> {
////        fn from_iter<T: IntoIterator<Item=I>>(_iter: T) -> Self {
////            //<::stdx::collections::impl_tests::MySet::<isize> as SetTestsisize>::test_all();
////            unimplemented!()
////        }
////    }
////
////    impl <T, Q> Set<Q> for MySet<T>
////        where T : Hash + Eq
////    {
////        fn contains(&self, _item: &T) -> bool {
////            unimplemented!()
////        }
////
////        fn get(&self, item: &Q) -> Option<&Self::Item> {
////            unimplemented!()
////        }
////
////        fn take(&mut self, item: &Q) -> Option<Self::Item> where Self: AddRemove {
////            unimplemented!()
////        }
////    }
////
////    impl <T> Iterator for MySet<T> {
////        type Item = T;
////
////        fn next(&mut self) -> Option<Self::Item> {
////            unimplemented!()
////        }
////    }
//
////    #[trait_tests] impl SetTestsisize for MySet<isize> {}
////    #[trait_tests] impl SetTestsfoo for MySet<Foo> {}
////    #[trait_tests] impl SetTestschar for MySet<char> {}
//}
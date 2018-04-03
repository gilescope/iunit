//use std::fmt::Debug;
//use std::collections::BTreeMap;
//
//use eclectic::{Map, AddRemove, Mutate};

//#[trait_tests]
//pub trait MapTests: Map<Item = (isize, isize), Key = isize, Value = isize>
//+ PartialEq
//+ Debug
//+ Sized
//+ AddRemove
//+ Mutate
//+  ::std::iter::FromIterator<isize>
////  + Index<usize, Output=isize> - not supported by linked list.
//{
//    fn new() -> Self;
//}
//
////Papercut: It seems base::Map is implemented for BTreeMap but not this last bit:
//impl Map for BTreeMap<isize, isize> {
//
//    fn get(&self, key: &isize) -> Option<&Self::Value> {
//        self.get(key)
//    }
//
//    fn get_mut(&mut self, key: &isize) -> Option<&mut Self::Value> where Self: Mutate {
//        self.get_mut(key)
//    }
//
//    fn remove(&mut self, key: &isize) -> Option<Self::Value> where Self: AddRemove {
//        self.remove(key)
//    }
//}
//
//#[trait_tests]
//impl MapTests for BTreeMap<isize, isize> { fn new() -> Self { BTreeMap::new() } }
//
//


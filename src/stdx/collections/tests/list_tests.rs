//use super::super::List;
use std::fmt::Debug;

use eclectic::{List, AddRemove, Mutate};

//pub struct DropCounter<'a> {
//    count: &'a mut u32,
//}
//
//impl<'a> Drop for DropCounter<'a> {
//    fn drop(&mut self) {
//        *self.count += 1;
//    }
//}
//
////TODO TwoVec<'b, T> unused type parameter.
//#[trait_tests]
//pub trait ListTests<'a>: List<DropCounter<'a>> + Sized {
//    fn test_double_drop() {
//        struct TwoVec<'b, T>
//            where T: ListTests<'b>
//        {
//            x: T,
//            y: T,
//           // z: ::std::marker::PhantomData<T>
//        }
//
//        let (mut count_x, mut count_y) = (0, 0);
//        {
//            let mut tv = TwoVec {
//                x: Self::new(),
//                y: Self::new(),
//            };
//            tv.x.push(DropCounter { count: &mut count_x });
//            tv.y.push(DropCounter { count: &mut count_y });
//
//            // If Vec had a drop flag, here is where it would be zeroed.
//            // Instead, it should rely on its internal state to prevent
//            // doing anything significant when dropped multiple times.
//            drop(tv.x);
//
//            // Here tv goes out of scope, tv.y should be dropped, but not tv.x.
//        }
//
//        assert_eq!(count_x, 1);
//        assert_eq!(count_y, 1);
//    }

#[trait_tests]
pub trait ListTestsBoxed: List<Item=Box<isize>>
+ PartialEq
+ Debug
+ Sized
+ AddRemove
+ Mutate
{
    fn new() -> Self;

    fn pop_back(&mut self) -> Option<Box<isize>> {
        self.pop()
    }

    fn pop_front(&mut self) -> Option<Box<isize>> {
        self.remove(0)
    }
    fn push_back(&mut self, item: Box<isize>) {
        self.push(item);
    }

    fn push_front(&mut self, item: Box<isize>) {
        self.insert(0, item)
    }

    fn test_basic() {
        //Other half of test_basic isize test.
        let mut m = Self::new();
        assert_eq!(m.pop_front(), None);
        assert_eq!(m.pop_back(), None);
        assert_eq!(m.pop_front(), None);
        m.push_front(box 1);
        assert_eq!(m.pop_front(), Some(box 1));
        m.push_back(box 2);
        m.push_back(box 3);
        assert_eq!(m.len(), 2);
        assert_eq!(m.pop_front(), Some(box 2));
        assert_eq!(m.pop_front(), Some(box 3));
        assert_eq!(m.len(), 0);
        assert_eq!(m.pop_front(), None);
        m.push_back(box 1);
        m.push_back(box 3);
        m.push_back(box 5);
        m.push_back(box 7);
        assert_eq!(m.pop_front(), Some(box 1));
    }
}



#[trait_tests]
pub trait ListTests: List<Item=isize>
    + PartialEq
    + Debug
    + Sized
    + AddRemove
    + Mutate
 +  ::std::iter::FromIterator<isize>
  //  + Index<usize, Output=isize> - not supported by linked list.
{
    fn new() -> Self;

    fn pop_back(&mut self) -> Option<isize> {
        self.pop()
    }

    fn pop_front(&mut self) -> Option<isize> {
        self.remove(0)
    }

    fn push_back(&mut self, item: isize) {
        self.push(item);
    }

    fn push_front(&mut self, item: isize) {
        self.insert(0, item);
    }

    fn front(&self) -> Option<&isize> {
        self.first()
    }
    fn back(&self) -> Option<&isize> {
        self.last()
    }
    fn front_mut(&mut self) -> Option<&mut isize> {
        self.first_mut()
    }
    fn back_mut(&mut self) -> Option<&mut isize> {
        self.last_mut()
    }
//    fn test_retain() {
//        //TODO WAS let mut vec = vec![1, 2, 3, 4];
//        let vec = &mut Self::new();
//        vec.push(1);
//        vec.push(2);
//        vec.push(3);
//        vec.push(4);
//
//        vec.retain(|&x| x % 2 == 0);
//
//        //TODO assert_eq!(vec, [2, 4]);
//
//        let res = vec.into_iter();
//        assert_eq!(res.next(), Some(2));
//        assert_eq!(res.next(), Some(4));
//        assert_eq!(res.next(), None);
//    }

    fn zero_sized_values() {
        let mut v = Self::new();
        assert_eq!(v.len(), 0);
        v.push(1);
        assert_eq!(v.len(), 1);
        v.push(1);
        assert_eq!(v.len(), 2);
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.pop(), None);

        assert_eq!(v.iter().count(), 0);
        v.push(1);
        assert_eq!(v.iter().count(), 1);
        v.push(1);
        assert_eq!(v.iter().count(), 2);

        for &_ in v.iter() {}

        assert_eq!(v.iter_mut().count(), 2);
        v.push(1);
        assert_eq!(v.iter_mut().count(), 3);
        v.push(1);
        assert_eq!(v.iter_mut().count(), 4);

        for &mut _ in &mut v.iter_mut() {}
        v.truncate(0);
        assert_eq!(v.iter_mut().count(), 0);
    }

    fn test_basic() {
        let mut n = Self::new();
        n.push_front(2);
        n.push_front(3);
        {
            assert_eq!(n.front().unwrap(), &3);
            let x = n.front_mut().unwrap();
            assert_eq!(*x, 3);
            *x = 0;
        }
        {
            assert_eq!(n.back().unwrap(), &2);
            let y = n.back_mut().unwrap();
            assert_eq!(*y, 2);
            *y = 1;
        }
        assert_eq!(n.pop_front(), Some(0));
        assert_eq!(n.pop_front(), Some(1));
    }


    //From liballoc/tests/linked_list.rs:
    fn list_from(v: &[isize]) -> Self {
        v.iter().cloned().collect()
    }

    fn generate_test() -> Self {
        Self::list_from(&[0, 1, 2, 3, 4, 5, 6])
    }

    fn test_split_off() {
        // singleton
        {
            let mut m = Self::new();
            m.push_back(1);

            let p = m.split_off(0);
            assert_eq!(m.len(), 0);
            assert_eq!(p.len(), 1);
            assert_eq!(p.back(), Some(&1));
            assert_eq!(p.front(), Some(&1));
        }

        // not singleton, forwards
        {
            let u = vec![1, 2, 3, 4, 5];
            let mut m = Self::list_from(&u);
            let mut n = m.split_off(2);
            assert_eq!(m.len(), 2);
            assert_eq!(n.len(), 3);
            for elt in 1..3 {
                assert_eq!(m.pop_front(), Some(elt));
            }
            for elt in 3..6 {
                assert_eq!(n.pop_front(), Some(elt));
            }
        }
        // not singleton, backwards
        {
            let u = vec![1, 2, 3, 4, 5];
            let mut m = Self::list_from(&u);
            let mut n = m.split_off(4);
            assert_eq!(m.len(), 4);
            assert_eq!(n.len(), 1);
            for elt in 1..5 {
                assert_eq!(m.pop_front(), Some(elt));
            }
            for elt in 5..6 {
                assert_eq!(n.pop_front(), Some(elt));
            }
        }

        // no-op on the last index
        {
            let mut m = Self::new();
            m.push_back(1);

            let p = m.split_off(1);
            assert_eq!(m.len(), 1);
            assert_eq!(p.len(), 0);
            assert_eq!(m.back(), Some(&1));
            assert_eq!(m.front(), Some(&1));
        }

    }

    fn test_iterator() {
        let m = Self::generate_test();
        for (i, elt) in m.iter().enumerate() {
            assert_eq!(i as isize, *elt);
        }
        let mut n = Self::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

//    fn test_iterator_clone() {
//        let mut n = Self::new();
//        n.push_back(2);
//        n.push_back(3);
//        n.push_back(4);
//        let mut it = n.iter();
//        it.next();
//        let mut jt = it.clone();
//        assert_eq!(it.next(), jt.next());
//        assert_eq!(it.next_back(), jt.next_back());
//        assert_eq!(it.next(), jt.next());
//    }
//
//    fn test_iterator_double_end() {
//        let mut n = Self::new();
//        assert_eq!(n.iter().next(), None);
//        n.push_front(4);
//        n.push_front(5);
//        n.push_front(6);
//        let mut it = n.iter();
//        assert_eq!(it.size_hint(), (3, Some(3)));
//        assert_eq!(it.next().unwrap(), &6);
//        assert_eq!(it.size_hint(), (2, Some(2)));
//        assert_eq!(it.next_back().unwrap(), &4);
//        assert_eq!(it.size_hint(), (1, Some(1)));
//        assert_eq!(it.next_back().unwrap(), &5);
//        assert_eq!(it.next_back(), None);
//        assert_eq!(it.next(), None);
//    }
//
//    fn test_rev_iter() {
//        let m = Self::generate_test();
//        for (i, elt) in m.iter().rev().enumerate() {
//            assert_eq!((6 - i) as isize, *elt);
//        }
//        let mut n = Self::new();
//        assert_eq!(n.iter().rev().next(), None);
//        n.push_front(4);
//        let mut it = n.iter().rev();
//        assert_eq!(it.size_hint(), (1, Some(1)));
//        assert_eq!(it.next().unwrap(), &4);
//        assert_eq!(it.size_hint(), (0, Some(0)));
//        assert_eq!(it.next(), None);
//    }
//
    fn test_mut_iter() {
        let mut m = Self::generate_test();
        let mut len = m.len();
        for (i, elt) in m.iter_mut().enumerate() {
            assert_eq!(i as isize, *elt);
            len -= 1;
        }
        assert_eq!(len, 0);
        let mut n = Self::new();
        assert!(n.iter_mut().next().is_none());
        n.push_front(4);
        n.push_back(5);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert!(it.next().is_some());
        assert!(it.next().is_some());
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert!(it.next().is_none());
    }
//
//    fn test_iterator_mut_double_end() {
//        let mut n = Self::new();
//        assert!(n.iter_mut().next_back().is_none());
//        n.push_front(4);
//        n.push_front(5);
//        n.push_front(6);
//        let mut it = n.iter_mut();
//        assert_eq!(it.size_hint(), (3, Some(3)));
//        assert_eq!(*it.next().unwrap(), 6);
//        assert_eq!(it.size_hint(), (2, Some(2)));
//        assert_eq!(*it.next_back().unwrap(), 4);
//        assert_eq!(it.size_hint(), (1, Some(1)));
//        assert_eq!(*it.next_back().unwrap(), 5);
//        assert!(it.next_back().is_none());
//        assert!(it.next().is_none());
//    }
//
//    fn test_mut_rev_iter() {
//        let mut m = Self::generate_test();
//        for (i, elt) in m.iter_mut().rev().enumerate() {
//            assert_eq!((6 - i) as isize, *elt);
//        }
//        let mut n = Self::new();
//        assert!(n.iter_mut().rev().next().is_none());
//        n.push_front(4);
//        let mut it = n.iter_mut().rev();
//        assert!(it.next().is_some());
//        assert!(it.next().is_none());
//    }
//
    fn test_eq() {
        let mut n = Self::list_from(&[]);
        let mut m = Self::list_from(&[]);
        assert!(n == m);
        n.push_front(1);
        assert!(n != m);
        m.push_back(1);
        assert!(n == m);

        let n = Self::list_from(&[2, 3, 4]);
        let m = Self::list_from(&[1, 2, 3]);
        assert!(n != m);
    }
//
//    //TODO maybe move to one with hash constraint
////    fn test_hash() {
////        let mut x = Self::new();
////        let mut y = Self::new();
////
////        assert!(::hash(&x) == ::hash(&y));
////
////        x.push_back(1);
////        x.push_back(2);
////        x.push_back(3);
////
////        y.push_front(3);
////        y.push_front(2);
////        y.push_front(1);
////
////        assert!(::hash(&x) == ::hash(&y));
////    }
//
//    fn test_ord() {
//        let n = Self::list_from(&[]);
//        let m = Self::list_from(&[1, 2, 3]);
//        assert!(n < m);
//        assert!(m > n);
//        assert!(n <= n);
//        assert!(n >= n);
//    }
//
//    fn test_ord_nan() {
//        let nan = 0.0f64 / 0.0;
//        let n = Self::list_from(&[nan]);
//        let m = Self::list_from(&[nan]);
//        assert!(!(n < m));
//        assert!(!(n > m));
//        assert!(!(n <= m));
//        assert!(!(n >= m));
//
//        let n = Self::list_from(&[nan]);
//        let one = Self::list_from(&[1.0f64]);
//        assert!(!(n < one));
//        assert!(!(n > one));
//        assert!(!(n <= one));
//        assert!(!(n >= one));
//
//        let u = Self::list_from(&[1.0f64, 2.0, nan]);
//        let v = Self::list_from(&[1.0f64, 2.0, 3.0]);
//        assert!(!(u < v));
//        assert!(!(u > v));
//        assert!(!(u <= v));
//        assert!(!(u >= v));
//
//        let s = Self::list_from(&[1.0f64, 2.0, 4.0, 2.0]);
//        let t = Self::list_from(&[1.0f64, 2.0, 3.0, 2.0]);
//        assert!(!(s < t));
//        assert!(s > one);
//        assert!(!(s <= one));
//        assert!(s >= one);
//    }
//
//    fn test_show() {
//        let list: Self = (0..10).collect();
//        assert_eq!(format!("{:?}", list), "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]");
//
//        let list: Self = vec!["just", "one", "test", "more"].iter().cloned().collect();
//        assert_eq!(format!("{:?}", list),
//                   "[\"just\", \"one\", \"test\", \"more\"]");
//    }
//
//    fn test_extend_ref() {
//        let mut a = Self::new();
//        a.push_back(1);
//
//        a.extend_objects(&[2, 3, 4]);
//
//        assert_eq!(a.len(), 4);
//        assert_eq!(a, Self::list_from(&[1, 2, 3, 4]));
//
//        let mut b = Self::new();
//        b.push_back(5);
//        b.push_back(6);
//        a.extend_objects(&b);
//
//        assert_eq!(a.len(), 6);
//        assert_eq!(a, Self::list_from(&[1, 2, 3, 4, 5, 6]));
//    }

//    fn test_extend() {
//        let mut a = Self::new();
//        a.push_back(1);
//        a.extend_object(vec![2, 3, 4]); // uses iterator
//
//        assert_eq!(a.len(), 4);
//        assert!(a.iter().eq(&[1, 2, 3, 4]));
//
//        let b: Self = vec![5, 6, 7].into_iter().collect();
//        a.extend_object(b); // specializes to `append`
//
//        assert_eq!(a.len(), 7);
//        assert!(a.iter().eq(&[1, 2, 3, 4, 5, 6, 7]));
//    }

//    fn test_contains() {
//        let mut l = Self::new();
//        l.extend_object(&[2usize, 3, 4]);
//
//        assert!(l.contains(&3));
//        assert!(!l.contains(&1));
//
//        l.clear();
//
//        assert!(!l.contains(&3));
//    }
//
//    fn drain_filter_empty() {
//        let mut list: Self = Self::new();
//
//        {
//            let mut iter = list.drain_filter(|_| true);
//            assert_eq!(iter.size_hint(), (0, Some(0)));
//            assert_eq!(iter.next(), None);
//            assert_eq!(iter.size_hint(), (0, Some(0)));
//            assert_eq!(iter.next(), None);
//            assert_eq!(iter.size_hint(), (0, Some(0)));
//        }
//
//        assert_eq!(list.len(), 0);
//        assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![]);
//    }
//
//    fn drain_filter_zst() {
//        let mut list: Self = vec![(), (), (), (), ()].into_iter().collect();
//        let initial_len = list.len();
//        let mut count = 0;
//
//        {
//            let mut iter = list.drain_filter(|_| true);
//            assert_eq!(iter.size_hint(), (0, Some(initial_len)));
//            while let Some(_) = iter.next() {
//                count += 1;
//                assert_eq!(iter.size_hint(), (0, Some(initial_len - count)));
//            }
//            assert_eq!(iter.size_hint(), (0, Some(0)));
//            assert_eq!(iter.next(), None);
//            assert_eq!(iter.size_hint(), (0, Some(0)));
//        }
//
//        assert_eq!(count, initial_len);
//        assert_eq!(list.len(), 0);
//        assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![]);
//    }
//
//    fn drain_filter_false() {
//        let mut list: Self = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter().collect();
//
//        let initial_len = list.len();
//        let mut count = 0;
//
//        {
//            let mut iter = list.drain_filter(|_| false);
//            assert_eq!(iter.size_hint(), (0, Some(initial_len)));
//            for _ in iter.by_ref() {
//                count += 1;
//            }
//            assert_eq!(iter.size_hint(), (0, Some(0)));
//            assert_eq!(iter.next(), None);
//            assert_eq!(iter.size_hint(), (0, Some(0)));
//        }
//
//        assert_eq!(count, 0);
//        assert_eq!(list.len(), initial_len);
//        assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
//    }
//
//    fn drain_filter_true() {
//        let mut list: Self = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter().collect();
//
//        let initial_len = list.len();
//        let mut count = 0;
//
//        {
//            let mut iter = list.drain_filter(|_| true);
//            assert_eq!(iter.size_hint(), (0, Some(initial_len)));
//            while let Some(_) = iter.next() {
//                count += 1;
//                assert_eq!(iter.size_hint(), (0, Some(initial_len - count)));
//            }
//            assert_eq!(iter.size_hint(), (0, Some(0)));
//            assert_eq!(iter.next(), None);
//            assert_eq!(iter.size_hint(), (0, Some(0)));
//        }
//
//        assert_eq!(count, initial_len);
//        assert_eq!(list.len(), 0);
//        assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![]);
//    }
//
//    fn drain_filter_complex() {
//
//        {   //                [+xxx++++++xxxxx++++x+x++]
//            let mut list = vec![
//                1,
//                2, 4, 6,
//                7, 9, 11, 13, 15, 17,
//                18, 20, 22, 24, 26,
//                27, 29, 31, 33,
//                34,
//                35,
//                36,
//                37, 39
//            ].into_iter().collect::<Self>();
//
//            let removed = list.drain_filter(|x| *x % 2 == 0).collect::<Vec<_>>();
//            assert_eq!(removed.len(), 10);
//            assert_eq!(removed, vec![2, 4, 6, 18, 20, 22, 24, 26, 34, 36]);
//
//            assert_eq!(list.len(), 14);
//            assert_eq!(
//                list.into_iter().collect::<Vec<_>>(),
//                vec![1, 7, 9, 11, 13, 15, 17, 27, 29, 31, 33, 35, 37, 39]
//            );
//        }
//
//        {   // [xxx++++++xxxxx++++x+x++]
//            let mut list = vec![
//                2, 4, 6,
//                7, 9, 11, 13, 15, 17,
//                18, 20, 22, 24, 26,
//                27, 29, 31, 33,
//                34,
//                35,
//                36,
//                37, 39
//            ].into_iter().collect::<Self>();
//
//            let removed = list.drain_filter(|x| *x % 2 == 0).collect::<Vec<_>>();
//            assert_eq!(removed.len(), 10);
//            assert_eq!(removed, vec![2, 4, 6, 18, 20, 22, 24, 26, 34, 36]);
//
//            assert_eq!(list.len(), 13);
//            assert_eq!(
//                list.into_iter().collect::<Vec<_>>(),
//                vec![7, 9, 11, 13, 15, 17, 27, 29, 31, 33, 35, 37, 39]
//            );
//        }
//
//        {   // [xxx++++++xxxxx++++x+x]
//            let mut list = vec![
//                2, 4, 6,
//                7, 9, 11, 13, 15, 17,
//                18, 20, 22, 24, 26,
//                27, 29, 31, 33,
//                34,
//                35,
//                36
//            ].into_iter().collect::<Self>();
//
//            let removed = list.drain_filter(|x| *x % 2 == 0).collect::<Vec<_>>();
//            assert_eq!(removed.len(), 10);
//            assert_eq!(removed, vec![2, 4, 6, 18, 20, 22, 24, 26, 34, 36]);
//
//            assert_eq!(list.len(), 11);
//            assert_eq!(
//                list.into_iter().collect::<Vec<_>>(),
//                vec![7, 9, 11, 13, 15, 17, 27, 29, 31, 33, 35]
//            );
//        }
//
//        {   // [xxxxxxxxxx+++++++++++]
//            let mut list = vec![
//                2, 4, 6, 8, 10, 12, 14, 16, 18, 20,
//                1, 3, 5, 7, 9, 11, 13, 15, 17, 19
//            ].into_iter().collect::<Self>();
//
//            let removed = list.drain_filter(|x| *x % 2 == 0).collect::<Vec<_>>();
//            assert_eq!(removed.len(), 10);
//            assert_eq!(removed, vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
//
//            assert_eq!(list.len(), 10);
//            assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19]);
//        }
//
//        {   // [+++++++++++xxxxxxxxxx]
//            let mut list = vec![
//                1, 3, 5, 7, 9, 11, 13, 15, 17, 19,
//                2, 4, 6, 8, 10, 12, 14, 16, 18, 20
//            ].into_iter().collect::<Self>();
//
//            let removed = list.drain_filter(|x| *x % 2 == 0).collect::<Vec<_>>();
//            assert_eq!(removed.len(), 10);
//            assert_eq!(removed, vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
//
//            assert_eq!(list.len(), 10);
//            assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19]);
//        }
//    }










}

#[trait_tests]
impl ListTests for Vec<isize>{ fn new() -> Self { Vec::new() } }
#[trait_tests]
impl ListTestsBoxed for Vec<Box<isize>>{ fn new() -> Self { Vec::new() } }

//TODO: Compiler error:
//#[trait_tests]
//impl ListTests for &[isize] { fn new() -> Self { Vec::new() } }

//TODO LinkedList doesn't implement List!
//impl ListTests for LinkedList<isize>{ fn new() -> Self { LinkedList::new() } }
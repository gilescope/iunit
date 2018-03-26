use super::super::List;
use std::fmt::Debug;
use std::collections::LinkedList;

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
pub trait ListTests: List<isize>
    + PartialEq
    + Debug + Sized
  //  + Index<usize, Output=isize> - not supported by linked list.
{
    fn a_Test() {}
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

//    fn zero_sized_values() {
//        let mut v = Self::new();
//        assert_eq!(v.len(), 0);
//        v.push(1);
//        assert_eq!(v.len(), 1);
//        v.push(1);
//        assert_eq!(v.len(), 2);
//        assert_eq!(v.pop(), Some(1));
//        assert_eq!(v.pop(), Some(1));
//        assert_eq!(v.pop(), None);
//
//        assert_eq!(v.iter().count(), 0);
//        v.push(1);
//        assert_eq!(v.iter().count(), 1);
//        v.push(1);
//        assert_eq!(v.iter().count(), 2);
//
//        for &() in &v {}
//
//        assert_eq!(v.iter_mut().count(), 2);
//        v.push(1);
//        assert_eq!(v.iter_mut().count(), 3);
//        v.push(1);
//        assert_eq!(v.iter_mut().count(), 4);
//
//        for &mut () in &mut v {}
//        unsafe {
//            v.set_len(0);
//        }
//        assert_eq!(v.iter_mut().count(), 0);
//    }
}

#[trait_tests]
impl ListTests for Vec<isize>{}

impl ListTests for LinkedList<isize>{}
use std::collections::BinaryHeap;
use eclectic::{PrioQueue, AddRemove};


#[trait_tests]
trait PriorityQueueTests : PrioQueue<Item=isize> + AddRemove + Sized + Clone //+ Mutate
{
    fn new() -> Self;

    fn test_iterator() {
        let mut data = vec![5, 9, 3];
        let iterout = [9, 5, 3];

        let mut pq = Self::new();
        pq.extend_object(&mut data.drain(..));

        let mut i = 0;
        for el in pq.iter() {
            assert_eq!(*el, iterout[i]);
            i += 1;
        }
    }

//    fn test_iterator_reverse() {
//        let mut data = vec![5, 9, 3];
//        let iterout = vec![3, 5, 9];
//
//        let mut pq = Self::new();
//        pq.extend_object(&mut data.drain(..));
//
//        let v: Vec<_> = pq.iter().rev().cloned().collect();
//        assert_eq!(v, iterout);
//    }

    fn test_peek_and_pop() {
        let mut data = vec![2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1];
        let mut sorted = data.clone();
        sorted.sort();

        let mut pq = Self::new();
        pq.extend_object(&mut data.drain(..));

        while !pq.is_empty() {
            assert_eq!(pq.front().unwrap(), sorted.last().unwrap());
            assert_eq!(pq.pop_front().unwrap(), sorted.pop().unwrap());
        }
    }

    fn test_push() {
        let mut data = vec![2, 4, 9];
        let mut pq = Self::new();
        pq.extend_object(&mut data.drain(..));
        assert_eq!(pq.len(), 3);

        assert_eq!(*pq.front().unwrap(), 9);
        pq.push(11);
        assert_eq!(pq.len(), 4);
        assert_eq!(*pq.front().unwrap(), 11);
        pq.push(5);
        assert_eq!(pq.len(), 5);
        assert_eq!(*pq.front().unwrap(), 11);
        pq.push(27);
        assert_eq!(pq.len(), 6);
        assert_eq!(*pq.front().unwrap(), 27);
        pq.push(3);
        assert_eq!(pq.len(), 7);
        assert_eq!(*pq.front().unwrap(), 27);
        pq.push(103);
        assert_eq!(pq.len(), 8);
        assert_eq!(*pq.front().unwrap(), 103);
    }

    fn check_to_vec(mut data: Vec<isize>) {
        let mut pq = Self::new();
        let mut data_clone = data.clone();
        pq.extend_object(&mut data_clone.drain(..));

        let mut v = pq.clone().into_vec();
        v.sort();
        data.sort();

        assert_eq!(v, data);
        //TODO next assert was testing into_sorted_vec(). into_vec doesn't guarantee any order.

        data.reverse();
        for expected in data {
            let actual = pq.pop_front();
            assert_eq!(actual, Some(expected));
        }
    }

    fn test_to_vec() {
        Self::check_to_vec(vec![]);
        Self::check_to_vec(vec![5]);
        Self::check_to_vec(vec![3, 2]);
        Self::check_to_vec(vec![2, 3]);
        Self::check_to_vec(vec![5, 1, 2]);
        Self::check_to_vec(vec![1, 100, 2, 3]);
        Self::check_to_vec(vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 0]);
        Self::check_to_vec(vec![2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1]);
        Self::check_to_vec(vec![9, 11, 9, 9, 9, 9, 11, 2, 3, 4, 11, 9, 0, 0, 0, 0]);
        Self::check_to_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        Self::check_to_vec(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
        Self::check_to_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 1, 2]);
        Self::check_to_vec(vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 1, 5, 4, 3, 2, 1]);
    }
}

#[trait_tests]
impl PriorityQueueTests for BinaryHeap<isize> { fn new() -> Self { BinaryHeap::new() } }
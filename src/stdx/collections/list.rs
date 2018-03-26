use std::collections::LinkedList;
//use core::ops::Index;

pub trait List<T> : IntoIterator<Item=T>
    where T : Sized
{
    fn new() -> Self;

    fn push(&mut self, value: T);

    fn retain<F>(&mut self, f: F)
        where F: FnMut(&T) -> bool;

    fn len(&self) -> usize;

    unsafe fn set_len(&mut self, len: usize);

    fn pop(&mut self) -> Option<T>;

 //   fn iter(&mut self) -> impl Iterator<T>;

//    fn iter_mut(&mut self) -> IterMut<T>
}


impl <T> List<T> for Vec<T> {
    fn new() -> Self {
        Vec::new()
    }

    fn push(&mut self, value: T) {
        self.push(value);
    }

    fn retain<F>(&mut self, f: F) where F: FnMut(&T) -> bool {
        self.retain(f)
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn pop(&mut self) -> Option<T> {
        self.pop()
    }
    unsafe fn set_len(&mut self, len: usize) {
        self.set_len(len)
    }
}

impl <T> List<T> for LinkedList<T> {
    fn new() -> Self {
        LinkedList::new()
    }

    fn push(&mut self, value: T) {
        self.push(value)
    }

    fn retain<F>(&mut self, f: F) where F: FnMut(&T) -> bool {
        self.retain(f)
    }
    fn len(&self) -> usize {
        self.len()
    }
    unsafe fn set_len(&mut self, len: usize) {
        self.set_len(len)
    }
    fn pop(&mut self) -> Option<T> {
        self.pop_front()
    }
}

//Can't implement as not in Index's crate...
//impl <T> Index<usize> for LinkedList<T> {
//    type Output = T;
//
//    fn index(&self, index: usize) -> &T {
//        &self.iter().skip(index).next()
//    }
//}

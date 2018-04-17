// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
//use std::heap::{Alloc, Layout};
//use trait_tests::trait_tests;
//
///// https://github.com/rust-lang/rust/issues/45955
/////
///// Note that `#[global_allocator]` is not used,
///// so `liballoc_jemalloc` is linked (on some platforms).
//
//#[trait_tests]
//trait AllocTests : Alloc + Sized {
//    fn test_check_overalign_requests(&mut self) {
//        let mut allocator = self;
//        let size = 8;
//        let align = 16; // greater than size
//        let iterations = 100;
//        unsafe {
//            let pointers: Vec<_> = (0..iterations).map(|_| {
//                allocator.alloc(Layout::from_size_align(size, align).unwrap()).unwrap()
//            }).collect();
//            for &ptr in &pointers {
//                assert_eq!((ptr as usize) % align, 0, "Got a pointer less aligned than requested")
//            }
//
//            // Clean up
//            for &ptr in &pointers {
//                allocator.dealloc(ptr, Layout::from_size_align(size, align).unwrap())
//            }
//        }
//    }
//}
//
//
//#[cfg(test)]
//mod test {
//    use super::*;
//    use alloc_system::System;
//    use std::alloc::Global;
//
//    //#[trait_tests] can't be used as System not defined in this crate.
//    impl AllocTests for System {}
//    //Can't do this either:
////    impl Default for System {
////        fn default() -> Self {
////            System::new()
////        }
////    }
//    //TODO new rethink.
//    #[test] fn test_alloctests_system() { <System as AllocTests>::test_all(System{});
////        <System as AllocTests>::test_instance(System {});
//    }
//
//    //#[trait_tests]
//    impl AllocTests for Global {}
//    #[test] fn test_alloctests_heap() { <Global as AllocTests>::test_all(Global{}) }
//}
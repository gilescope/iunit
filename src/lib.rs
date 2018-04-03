//!
//! goals:
//!
//! [x] Trait linked to TraitTests
//!
//! [x] Impl annotation creates tests.
//!
//! additional:
//!
//! [x] Tests covering multiple Traits
//!
//! [ ] Each test is counted as a unique test. (Currently each set of trait tests counts as only 1 test).
//!
#![feature(custom_attribute)]
#![feature(plugin)]

#![feature(box_syntax)]
#![feature(placement_in_syntax)]

#![feature(test)]//for benches

#![feature(alloc_system)] //alloc tests
#![feature(allocator_api)]

#![plugin(trait_tests)]

extern crate eclectic;
//priority panic tests
extern crate rand;
extern crate test;

//alloc tests
extern crate alloc_system;

pub mod stdx;
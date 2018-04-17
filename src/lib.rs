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
#![feature(box_syntax)]
#![feature(placement_in_syntax)]

#![feature(test)]//for benches

#![feature(alloc_system)] //alloc tests
#![feature(allocator_api)]

#![feature(proc_macro)]
#![macro_use] extern crate trait_tests;

#[cfg(feature="eclectic_tests")]
extern crate eclectic;
//priority panic tests

//extern crate rand;
//bench:
//extern crate test;

//alloc tests
extern crate alloc_system;

extern crate proptest;

#[cfg(feature="num_tests")]
extern crate num_traits;

#[cfg(all(feature="num_tests", test))]
extern crate num_complex;
#[cfg(all(feature="num_tests", test))]
extern crate num_bigint;
#[cfg(all(feature="num_tests", test))]
extern crate num_integer;
#[cfg(all(feature="num_tests", test))]
extern crate num_rational;


#[cfg(feature="dimensioned")]
extern crate dimensioned;

#[cfg(feature="dimensioned")]
pub mod dimensioned_tests;

pub mod stdx;
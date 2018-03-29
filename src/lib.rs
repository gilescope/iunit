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

#![plugin(trait_tests)]

extern crate eclectic;
extern crate rand;

pub mod stdx;
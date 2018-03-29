# Trait Tests

## Why

More tests are great, but less code is less bugs so we want more tests but less code. This crate attempts to break the N*M problem of repeatedly writing tests for all the individual implementations. 

If we can agree on standard 'conventional' tests that one would expect a 'Set' implementation to adhere to then we only have to code up the tests for the interesting additional functionality.

At no point is an implementation forced to adhere to these tests. There are always special cases / pathological implementations.

My dream is of a std library that ships with std tests, 
and gradually an ecosystem of people publishing std tests with their interfaces.

Warning: This is a proof of concept.

# Free Tests

iunit comprises of the following free tests just for you:

   * List: 1 tests.
   * Set: 7 tests.
   * PriorityQueue: 10 tests.

## How: Defining a test on a Trait

To create a trait test, create a subtrait of the trait under test with static functions on it. The generic parameters should be concreted out into a type of your choosing to help you with the testing.

```rust
#[trait_tests]
pub trait SetIteratorTests: Set<char> + Sized + IntoIterator<Item=char>
{
    fn test_move_iter()
    {
        let hs = {
            let mut hs = Self::new();
            hs.insert('a');
            hs.insert('b');
            hs
        };

        let v = hs.into_iter().collect::<Vec<char>>();
        assert!(v == ['a', 'b'] || v == ['b', 'a']);
    }
}
```
You can have as many test functions as you like, but you need one subtrait for each set of type parameters you wish to test. At the moment all the tests defined on an interface will be reported as one test, but when test failures occur the stack trace makes it clear which test failed.

## How: Testing your implementation

To test your implementation you would include the following in the crate where you define your implementation. (Rust's impl restrictions mean it can't be defined anywhere else.)

```rust
#[trait_tests] impl SetIteratorTests for HashSet<isize> {}
```

The compiler would guides you as to what type parameters you have to put in your implementation. (It won't figure out HashSet<_> alas.)

The compiler will also guide you in informing you of additional traits that the particular test would need to have implemented in order to function. As certain traits go together this is a nice way of ensuring your implementation is well-rounded.

## Installing

The compiler plugin is in the 'trait_tests' crate.

The std interface tests are currently defined in iunit.

```toml
[dependencies]
trait_tests = "*"
iunit = "*"
```

## Example Trait Tests

To demonstrate the power of trait tests we need some traits. 

The eclectic crate ( https://github.com/apasel422/eclectic ) defines a set of collection traits for the standard collections. 
(Collections traits were in Rust pre 1.0 but were not ready for inclusion into v1.0 so were removed.)

In this repository you will find some of the Rust collection tests recycled to exercise the collection traits.

## Open Questions

  1. How do we get the test framework to enumerate all the individual tests.
  2. Can the iunit crate be only pulled in as a dev-dependency?
  3. Does anyone find this useful?
  
All feedback / help / contributions extremely welcome!
mod set_tests;
mod list_tests;
mod queue_tests;
mod map_tests;

pub use self::map_tests::*;
pub use self::set_tests::*;
pub use self::list_tests::*;
pub use self::queue_tests::*;

use eclectic::Collection;

#[trait_tests]
pub trait CollectionTests : Collection + Sized {
    fn new() -> Self;

    fn test_new_length() {
        assert_eq!(Self::new().len(), 0);
    }
}
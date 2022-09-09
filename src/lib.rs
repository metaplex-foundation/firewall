mod plan;
mod account_info;
mod constraints;
mod account_constraints;
mod error;

use std::cell::RefMut;
use crate::constraints::Constraints;
pub use error::*;
pub use plan::*;
pub use constraints::*;
pub use account_constraints::*;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

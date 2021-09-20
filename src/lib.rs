#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::all, clippy::pedantic)]
#![cfg_attr(feature = "strict", deny(warnings))]

//! Defines various map-related enums, structs, and types (traits are contained in `dungen_minion_rooms_abstract`) for the `dungen_minion` crate.
//!
//! As the purpose of this crate is to provide concrete data types for `dungen_minion` and `dungen_minion`'s other dependent crates to consume, only implementation-specific details are defined here, while their general usages are usually defined in `dungen_minion_rooms_abstract`, and specific other details in `dungen_minion_geometry`.

// External includes.
pub use dungen_minion_rooms_abstract::*;

// Standard includes.

// Internal includes.
mod sparse_map;

pub use sparse_map::SparseMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

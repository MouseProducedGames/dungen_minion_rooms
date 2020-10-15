#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! Defines various map-related enums, structs, and types (traits are contained in `dungen_minion_rooms_abstract`) for the `dungen_minion` crate.
//!
//! As the purpose of this crate is to provide concrete data types for `dungen_minion` and `dungen_minion`'s other dependent crates to consume, only implementation-specific details are defined here, while their general usages are usually defined in `dungen_minion_rooms_abstract`, and specific other details in `dungen_minion_geometry`.

// External includes.
pub use dungen_minion_rooms_abstract::*;

// Standard includes.

// Internal includes.
mod map_sparse;

pub use map_sparse::SparseMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

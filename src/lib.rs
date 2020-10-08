// External includes.
pub use dungen_minion_rooms_abstract::*;

// Standard includes.

// Internal includes.
mod room_hash_map;

pub use room_hash_map::RoomHashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

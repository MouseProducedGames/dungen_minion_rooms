// External includes.
use super::{
    PlacedRoom, Portal, PortalCollection, Portals, PortalsMut, Room, SubRoom, SubRoomCollection,
    SubRooms, SubRoomsMut, TileType,
};
use crate::geometry::*;

// Standard includes.
use std::collections::HashMap;
use std::default::Default;

// Internal includes.

/// A room which stores its [`TileType`](enum.TileType.html) information in a `HashMap`, indexed by [`ShapePosition`](geometry/struct.ShapePosition.html).
///
/// The size of the `RoomHashMap` will expand based on the `ShapePosition` provided, as per the specification for [`Room`](trait.Room.html).
#[derive(Clone)]
pub struct RoomHashMap {
    shape_position: ShapePosition,
    size: Size,
    tiles: HashMap<ShapePosition, TileType>,
    portals: Vec<Portal>,
    sub_rooms: Vec<SubRoom>,
}

impl RoomHashMap {
    /// Creates a new `RoomHashMap`. As `RoomHashMap` expands to meet its use, no parameters need be supplied.
    ///
    /// `RoomHashMap::default()` defers to `RoomHashMap::new()`.
    pub fn new() -> Self {
        Self {
            shape_position: ShapePosition::new(0, 0),
            size: Size::new(0, 0),
            tiles: HashMap::new(),
            portals: Vec::new(),
            sub_rooms: Vec::new(),
        }
    }
}

impl Default for RoomHashMap {
    fn default() -> Self {
        Self::new()
    }
}

impl HasShapePosition for RoomHashMap {
    fn shape_position(&self) -> &ShapePosition {
        &self.shape_position
    }

    fn shape_position_mut(&mut self) -> &mut ShapePosition {
        &mut self.shape_position
    }
}

impl IntersectsShapePosition for RoomHashMap {
    fn intersects_shape_position(&self, pos: ShapePosition) -> bool {
        self.tiles.contains_key(&pos)
    }
}

impl PortalCollection for RoomHashMap {
    fn add_portal(
        &mut self,
        local_shape_position: ShapePosition,
        portal_to_room_facing: OrdinalDirection,
        target: Box<dyn PlacedRoom>,
    ) {
        self.portals.push(Portal::new(
            local_shape_position,
            portal_to_room_facing,
            target,
        ));
        self.tile_type_at_local_set(local_shape_position, TileType::Portal);
    }

    fn get_portal_at(&self, index: usize) -> Option<&Portal> {
        self.portals.get(index)
    }

    fn get_portal_at_mut(&mut self, index: usize) -> Option<&mut Portal> {
        self.portals.get_mut(index)
    }

    fn portal_count(&self) -> usize {
        self.portals.len()
    }
}

impl Room for RoomHashMap {
    fn box_clone(&self) -> Box<dyn Room> {
        Box::new((*self).clone())
    }

    fn portals(&self) -> Portals {
        Portals::new(&self.portals)
    }
    fn portals_mut(&mut self) -> PortalsMut {
        PortalsMut::new(&mut self.portals)
    }

    fn sub_rooms(&self) -> SubRooms {
        SubRooms::new(&self.sub_rooms)
    }

    fn sub_rooms_mut(&mut self) -> SubRoomsMut {
        SubRoomsMut::new(&mut self.sub_rooms)
    }

    fn tile_type_at_local(&self, pos: ShapePosition) -> Option<&TileType> {
        self.tiles.get(&pos)
    }

    fn tile_type_at_local_mut(&mut self, pos: ShapePosition) -> Option<&mut TileType> {
        self.tiles.get_mut(&pos)
    }

    fn tile_type_at_local_set(
        &mut self,
        pos: ShapePosition,
        tile_type: TileType,
    ) -> Option<TileType> {
        *self.size_mut().height_mut() = self.size.height().max(pos.y() as u32 + 1);
        *self.size_mut().width_mut() = self.size.width().max(pos.x() as u32 + 1);

        self.tiles.insert(pos, tile_type)
    }
}

impl Shape for RoomHashMap {}

impl SubRoomCollection for RoomHashMap {
    fn add_sub_room(&mut self, local_shape_position: ShapePosition, target: Box<dyn Room>) {
        self.sub_rooms
            .push(SubRoom::new(local_shape_position, target))
    }

    fn get_sub_room_at(&self, index: usize) -> Option<&SubRoom> {
        self.sub_rooms.get(index)
    }

    fn get_sub_room_at_mut(&mut self, index: usize) -> Option<&mut SubRoom> {
        self.sub_rooms.get_mut(index)
    }

    fn sub_room_count(&self) -> usize {
        self.sub_rooms.len()
    }
}

impl HasSize for RoomHashMap {
    fn size(&self) -> &Size {
        &self.size
    }

    fn size_mut(&mut self) -> &mut Size {
        &mut self.size
    }
}

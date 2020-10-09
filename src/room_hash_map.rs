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

#[derive(Clone)]
pub struct RoomHashMap {
    local: LocalPosition,
    size: Size,
    tiles: HashMap<LocalPosition, TileType>,
    portals: Vec<Portal>,
    sub_rooms: Vec<SubRoom>,
}

impl RoomHashMap {
    pub fn new() -> Self {
        Self {
            local: LocalPosition::new(0, 0),
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

impl HasLocalPosition for RoomHashMap {
    fn local(&self) -> &LocalPosition {
        &self.local
    }

    fn local_mut(&mut self) -> &mut LocalPosition {
        &mut self.local
    }
}

impl IntersectsLocalPos for RoomHashMap {
    fn intersects_local_pos(&self, pos: LocalPosition) -> bool {
        self.tiles.contains_key(&pos)
    }
}

impl PortalCollection for RoomHashMap {
    fn add_portal(
        &mut self,
        local: LocalPosition,
        portal_to_room_facing: OrdinalDirection,
        target: Box<dyn PlacedRoom>,
    ) {
        self.portals
            .push(Portal::new(local, portal_to_room_facing, target));
        self.tile_type_at_local_set(local, TileType::Portal);
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

    fn tile_type_at_local(&self, pos: LocalPosition) -> Option<&TileType> {
        self.tiles.get(&pos)
    }

    fn tile_type_at_local_mut(&mut self, pos: LocalPosition) -> Option<&mut TileType> {
        self.tiles.get_mut(&pos)
    }

    fn tile_type_at_local_set(
        &mut self,
        pos: LocalPosition,
        tile_type: TileType,
    ) -> Option<TileType> {
        *self.size_mut().height_mut() = self.size.height().max(pos.y() + 1);
        *self.size_mut().width_mut() = self.size.width().max(pos.x() + 1);

        self.tiles.insert(pos, tile_type)
    }
}

impl Shape for RoomHashMap {}

impl SubRoomCollection for RoomHashMap {
    fn add_sub_room(&mut self, local: LocalPosition, target: Box<dyn Room>) {
        self.sub_rooms.push(SubRoom::new(local, target))
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

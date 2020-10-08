// External includes.
use super::{Portal, Portals, PortalsMut, Room, SubRoom, SubRooms, SubRoomsMut, TileType};
use crate::geometry::{HasLocalPosition, HasSize, IntersectsLocalPos, LocalPosition, Shape, Size};

// Standard includes.
use std::collections::HashMap;
use std::default::Default;

// Internal includes.

#[derive(Clone)]
pub struct RoomHashMap<'a> {
    local: LocalPosition,
    size: Size,
    tiles: HashMap<LocalPosition, TileType>,
    portals: Vec<Portal<'a>>,
    sub_rooms: Vec<SubRoom<'a>>,
}

impl<'a> RoomHashMap<'a> {
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

impl<'a> Default for RoomHashMap<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> HasLocalPosition for RoomHashMap<'a> {
    fn local(&self) -> &LocalPosition {
        &self.local
    }

    fn local_mut(&mut self) -> &mut LocalPosition {
        &mut self.local
    }
}

impl<'a> IntersectsLocalPos for RoomHashMap<'a> {
    fn intersects_local_pos(&self, pos: LocalPosition) -> bool {
        self.tiles.contains_key(&pos)
    }
}

impl<'a> Room<'a> for RoomHashMap<'a> {
    fn portals(&'a self) -> Portals<'a> {
        Portals::new(&self.portals)
    }
    fn portals_mut(&'a mut self) -> PortalsMut<'a> {
        PortalsMut::new(&mut self.portals)
    }

    fn sub_rooms(&'a self) -> SubRooms<'a> {
        SubRooms::new(&self.sub_rooms)
    }

    fn sub_rooms_mut(&'a mut self) -> SubRoomsMut<'a> {
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
        *self.size.height_mut() = self.size.height().max(pos.y() + 1);
        *self.size.width_mut() = self.size.width().max(pos.x() + 1);

        self.tiles.insert(pos, tile_type)
    }
}

impl<'a> Shape for RoomHashMap<'a> {}

impl<'a> HasSize for RoomHashMap<'a> {
    fn size(&self) -> &Size {
        &self.size
    }

    fn size_mut(&mut self) -> &mut Size {
        &mut self.size
    }
}

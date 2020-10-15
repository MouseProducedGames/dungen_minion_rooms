// External includes.
use super::{
    get_new_map_id, register_map, Map, MapId, Portal, PortalCollection, Portals, PortalsMut,
    SubMap, SubMapCollection, SubMaps, SubMapsMut, TileType,
};
use crate::geometry::*;

// Standard includes.
use std::collections::HashMap;

// Internal includes.

/// A map which stores its [`TileType`](enum.TileType.html) information in a `HashMap`, indexed by [`Position`](geometry/struct.Position.html).
///
/// The size of the `MapSparse` will expand based on the `Position` provided, as per the specification for [`Map`](trait.Map.html).
#[derive(Clone)]
pub struct MapSparse {
    map_id: MapId,
    area: Area,
    tiles: HashMap<Position, TileType>,
    portals: Vec<Portal>,
    sub_maps: Vec<SubMap>,
}

impl MapSparse {
    /// Creates a new `MapSparse`. As `MapSparse` expands to meet its use, no parameters need be supplied.
    ///
    /// `MapSparse::default()` defers to `MapSparse::new()`.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> MapId {
        register_map(Self {
            map_id: get_new_map_id(),
            area: Area::new(Position::zero(), Size::zero()),
            tiles: HashMap::new(),
            portals: Vec::new(),
            sub_maps: Vec::new(),
        })
    }
}

impl HasArea for MapSparse {
    fn area(&self) -> &Area {
        &self.area
    }

    fn area_mut(&mut self) -> &mut Area {
        &mut self.area
    }
}

impl HasPosition for MapSparse {
    fn position(&self) -> &Position {
        self.area.position()
    }

    fn position_mut(&mut self) -> &mut Position {
        self.area.position_mut()
    }
}

impl IntersectsPos for MapSparse {
    fn intersects_pos(&self, position: Position) -> bool {
        !(position.x() < self.position().x()
            || position.y() < self.position().y()
            || position.x() >= self.area.right()
            || position.y() >= self.area.bottom())
    }
}

impl Placed for MapSparse {}

impl PlacedObject for MapSparse {}

impl PlacedShape for MapSparse {}

impl PortalCollection for MapSparse {
    fn add_portal(
        &mut self,
        local_position: Position,
        portal_to_map_facing: OrdinalDirection,
        portal_to_map_position: Position,
        target: MapId,
    ) {
        self.portals.push(Portal::new(
            local_position,
            portal_to_map_facing,
            portal_to_map_position,
            target,
        ));
        self.tile_type_at_local_set(local_position, TileType::Portal);
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

    fn portals(&self) -> Portals {
        Portals::new(&self.portals)
    }

    fn portals_mut(&mut self) -> PortalsMut {
        PortalsMut::new(&mut self.portals)
    }
}

impl Map for MapSparse {
    fn box_clone(&self) -> Box<dyn Map> {
        Box::new((*self).clone())
    }

    fn map_id(&self) -> MapId {
        self.map_id
    }

    fn tile_type_at_local(&self, pos: Position) -> Option<&TileType> {
        self.tiles.get(&pos)
    }

    fn tile_type_at_local_mut(&mut self, pos: Position) -> Option<&mut TileType> {
        self.tiles.get_mut(&pos)
    }

    fn tile_type_at_local_set(&mut self, pos: Position, tile_type: TileType) -> Option<TileType> {
        *self.size_mut().height_mut() = self.size().height().max(pos.y() as u32 + 1);
        *self.size_mut().width_mut() = self.size().width().max(pos.x() as u32 + 1);

        self.tiles.insert(pos, tile_type)
    }
}

impl Shape for MapSparse {}

impl SubMapCollection for MapSparse {
    fn add_sub_map(&mut self, local_position: Position, target: MapId) {
        self.sub_maps.push(SubMap::new(local_position, target))
    }

    fn get_sub_map_at(&self, index: usize) -> Option<&SubMap> {
        self.sub_maps.get(index)
    }

    fn get_sub_map_at_mut(&mut self, index: usize) -> Option<&mut SubMap> {
        self.sub_maps.get_mut(index)
    }

    fn sub_map_count(&self) -> usize {
        self.sub_maps.len()
    }

    fn sub_maps(&self) -> SubMaps {
        SubMaps::new(&self.sub_maps)
    }

    fn sub_maps_mut(&mut self) -> SubMapsMut {
        SubMapsMut::new(&mut self.sub_maps)
    }
}

impl HasSize for MapSparse {
    fn size(&self) -> &Size {
        self.area.size()
    }

    fn size_mut(&mut self) -> &mut Size {
        self.area.size_mut()
    }
}

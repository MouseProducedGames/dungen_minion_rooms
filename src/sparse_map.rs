// External includes.
use super::{
    get_new_map_id, register_map, Map, MapId, Portal, PortalCollection, Portals, PortalsMut,
    SubMap, SubMapCollection, SubMaps, SubMapsMut, TileType, TileTypeCmp, TileTypeStandardCmp,
    MAPS,
};
use crate::geometry::*;

// Standard includes.
use std::collections::HashMap;

// Internal includes.

/// A map which stores its [`TileType`](enum.TileType.html) information in a `HashMap`, indexed by [`Position`](geometry/struct.Position.html).
///
/// The size of the `SparseMap` will expand based on the `Position` provided, as per the specification for [`Map`](trait.Map.html).
#[derive(Clone)]
pub struct SparseMap {
    map_id: MapId,
    area: Area,
    tiles: HashMap<Position, TileType>,
    portals: Vec<Portal>,
    sub_maps: Vec<SubMap>,
}

impl SparseMap {
    /// Creates a new `SparseMap`. As `SparseMap` expands to meet its use, no parameters need be supplied.
    ///
    /// `SparseMap::default()` defers to `SparseMap::new()`.
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

impl HasArea for SparseMap {
    fn area(&self) -> &Area {
        &self.area
    }

    fn area_mut(&mut self) -> &mut Area {
        &mut self.area
    }
}

impl HasPosition for SparseMap {
    fn position(&self) -> &Position {
        self.area.position()
    }

    fn position_mut(&mut self) -> &mut Position {
        self.area.position_mut()
    }
}

impl HasSize for SparseMap {
    fn size(&self) -> &Size {
        self.area.size()
    }

    fn size_mut(&mut self) -> &mut Size {
        self.area.size_mut()
    }
}

impl IntersectsPos for SparseMap {
    fn intersects_pos(&self, position: Position) -> bool {
        !(position.x() < self.position().x()
            || position.y() < self.position().y()
            || position.x() >= self.area.right()
            || position.y() >= self.area.bottom())
    }
}

impl Map for SparseMap {
    fn box_clone(&self) -> Box<dyn Map> {
        Box::new((*self).clone())
    }

    fn map_id(&self) -> MapId {
        self.map_id
    }

    fn tile_type_at_local(&self, pos: Position) -> Option<TileType> {
        let mut output = None;
        if !self.sub_maps.is_empty() {
            let maps = MAPS.read();
            for sub_map in self.sub_maps.iter() {
                let map = maps[sub_map.value()].read();
                // println!("*sub_map.local_position() {}", *sub_map.local_position());
                let sub_map_position = *sub_map.local_position();
                let local_position = pos - sub_map_position;
                let test = map.tile_type_at_local(local_position);
                output = *TileTypeStandardCmp::return_greater_option(&output, &test);
            }
        }

        let self_tile_type = if let Some(tile_type) = self.tiles.get(&pos) {
            Some(*tile_type)
        } else {
            None
        };
        output = *TileTypeStandardCmp::return_greater_option(&output, &self_tile_type);

        output
    }

    fn tile_type_at_local_mut(&mut self, pos: Position) -> Option<&mut TileType> {
        self.tiles.get_mut(&pos)
    }

    fn tile_type_at_local_set(&mut self, pos: Position, tile_type: TileType) -> Option<TileType> {
        if !self.sub_maps.is_empty() {
            let maps = MAPS.read();
            for sub_map in self.sub_maps.iter() {
                let mut map = maps[sub_map.value()].write();
                let sub_map_position = *sub_map.local_position();
                let local_position = pos - sub_map_position;
                if map.is_local_position_valid(local_position) {
                    map.tile_type_at_local_set(local_position, tile_type);
                }
            }
        }

        *self.size_mut().height_mut() = self.size().height().max(pos.y() as u32 + 1);
        *self.size_mut().width_mut() = self.size().width().max(pos.x() as u32 + 1);

        self.tiles.insert(pos, tile_type)
    }

    /// Gets an option for an immutable reference to the `TileType` at the given local `Position`. Returns None if the local `Position` is out of bounds, or there is no tile at that location.
    ///
    /// Uses a comparison function to determine which sub-map tile has priority, if any.
    fn tile_type_at_local_sort_by<'a>(
        &self,
        pos: Position,
        sort_best: &dyn Fn(&Option<TileType>, &Option<TileType>) -> std::cmp::Ordering,
    ) -> Option<TileType> {
        let mut output = None;
        if !self.sub_maps.is_empty() {
            let maps = MAPS.read();
            for sub_map in self.sub_maps.iter() {
                let map = maps[sub_map.value()].read();
                // println!("*sub_map.local_position() {}", *sub_map.local_position());
                let sub_map_position = *sub_map.local_position();
                let local_position = pos - sub_map_position;
                let test = map.tile_type_at_local(local_position);
                output = match sort_best(&output, &test) {
                    std::cmp::Ordering::Greater => output,
                    std::cmp::Ordering::Equal => output,
                    std::cmp::Ordering::Less => test,
                };
            }
        }

        let self_tile_type = if let Some(tile_type) = self.tiles.get(&pos) {
            Some(*tile_type)
        } else {
            None
        };
        output = match sort_best(&output, &self_tile_type) {
            std::cmp::Ordering::Greater => output,
            std::cmp::Ordering::Equal => output,
            std::cmp::Ordering::Less => self_tile_type,
        };

        output
    }
}

impl Placed for SparseMap {}

impl PlacedObject for SparseMap {}

impl PlacedShape for SparseMap {}

impl PortalCollection for SparseMap {
    fn add_portal(
        &mut self,
        local_position: Position,
        portal_to_map_facing: CardinalDirection,
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

impl Shape for SparseMap {}

impl SubMapCollection for SparseMap {
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

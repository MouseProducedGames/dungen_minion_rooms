// External includes.
use super::{
    get_new_map_id, register_map, Map, MapId, Portal, PortalCollection, Portals, PortalsMut,
    SubMap, SubMapCollection, SubMaps, SubMapsMut, TileType, TileTypeCmp, TileTypeStandardCmp,
    MAPS,
};

// Standard includes.
use std::collections::HashMap;

// Internal includes.
use crate::geometry::{
    Area, CardinalDirection, CardinalRotation, Containment, ContainsLocalPosition,
    ContainsPosition, HasArea, HasBottom, HasHeight, HasPosition, HasRight, HasSize, HasWidth,
    IntersectsLocalPosition, IntersectsPosition, IsPosition, Length, Placed, PlacedObject,
    Position, Shape, ShapeIterator, Size,
};

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
    #[must_use = "If the `MapId` is not stored, the `Map` is lost."]
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

impl ContainsLocalPosition for SparseMap {
    /// ```
    /// # use dungen_minion_rooms::geometry::*;
    /// # use dungen_minion_rooms::*;
    /// # let map_id = SparseMap::new();
    ///
    /// # let maps = MAPS.read();
    /// # let mut sparse_map = maps[map_id].write();
    /// for y in -1..=1 {
    ///     for x in -1..=1 {
    ///         let position = Position::new(2, 2) + Position::new(x, y);
    ///         sparse_map.tile_type_at_local_set(position, TileType::Wall);
    ///     }
    /// }
    ///
    /// for y in -2..=2 {
    ///     for x in -2..=2 {
    ///         let position = Position::new(2, 2) + Position::new(x, y);
    ///         let containment = sparse_map.contains_local_position(position);
    ///         if x.abs() == 2 || y.abs() == 2 {
    ///             assert!(containment == Containment::Disjoint);
    ///         } else if x.abs() == 1 || y.abs() == 1 {
    ///             assert!(containment == Containment::Intersects);
    ///         } else {
    ///             assert!(containment == Containment::Contains);
    ///         }
    ///     }
    /// }
    /// ```
    fn contains_local_position(&self, position: Position) -> Containment {
        if self.intersects_local_position(position) {
            if self.intersects_local_position(position + Position::NORTH)
                && self.intersects_local_position(position + Position::NORTH + Position::EAST)
                && self.intersects_local_position(position + Position::EAST)
                && self.intersects_local_position(position + Position::SOUTH + Position::EAST)
                && self.intersects_local_position(position + Position::SOUTH)
                && self.intersects_local_position(position + Position::SOUTH + Position::WEST)
                && self.intersects_local_position(position + Position::WEST)
                && self.intersects_local_position(position + Position::NORTH + Position::WEST)
            {
                Containment::Contains
            } else {
                Containment::Intersects
            }
        } else {
            Containment::Disjoint
        }
    }
}

impl ContainsPosition for SparseMap {}

impl HasArea for SparseMap {
    fn area(&self) -> &Area {
        &self.area
    }

    fn area_mut(&mut self) -> &mut Area {
        &mut self.area
    }
}

impl HasHeight for SparseMap {
    fn height(&self) -> Length {
        self.size().height()
    }

    fn height_mut(&mut self) -> &mut Length {
        self.size_mut().height_mut()
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

impl HasWidth for SparseMap {
    fn width(&self) -> Length {
        self.size().width()
    }

    fn width_mut(&mut self) -> &mut Length {
        self.size_mut().width_mut()
    }
}

impl IntersectsLocalPosition for SparseMap {
    fn intersects_local_position(&self, position: Position) -> bool {
        self.area().intersects_local_position(position)
            && !matches!(
                self.tile_type_at_local(position),
                Some(TileType::Void) | None
            )
    }
}

impl IntersectsPosition for SparseMap {}

impl Map for SparseMap {
    fn box_clone(&self) -> Box<dyn Map> {
        Box::new((*self).clone())
    }

    fn map_id(&self) -> MapId {
        self.map_id
    }

    fn rotate(&mut self, rotation: CardinalRotation) {
        /* if rotation == CardinalRotation::None {
            return;
        } */

        if self.area().is_empty() {
            return;
        }

        let mut new_tiles = HashMap::<Position, TileType>::new();

        let self_position = *self.position();
        let new_self_position = self_position * rotation;
        let adjust_position = match rotation {
            CardinalRotation::None => Position::new(0, 0),
            // Wrap-around is only possible with implausibly large maps.
            #[allow(clippy::cast_possible_wrap)]
            CardinalRotation::Right90 => Position::new(0, (self.area().width() as i32 - 1).max(0)),
            // Wrap-around is only possible with implausibly large maps.
            #[allow(clippy::cast_possible_wrap)]
            CardinalRotation::Full180 => Position::new(
                (self.area().width() as i32 - 1).max(0),
                (self.area().height() as i32 - 1).max(0),
            ),
            // Wrap-around is only possible with implausibly large maps.
            #[allow(clippy::cast_possible_wrap)]
            CardinalRotation::Left90 => Position::new((self.area().height() as i32 - 1).max(0), 0),
        };

        for portal_mut in &mut self.portals {
            let portal_local_position = *portal_mut.local_position() - self_position;
            let new_portal_local_position = portal_local_position * rotation;
            let new_portal_position = new_self_position + new_portal_local_position;
            *portal_mut.local_position_mut() = adjust_position + new_portal_position;
        }

        for kvp in &self.tiles {
            let tile_local_position = *kvp.0 - self_position;
            let new_tile_local_position = tile_local_position * rotation;
            let new_tile_position = new_self_position + new_tile_local_position;
            new_tiles.insert(adjust_position + new_tile_position, *kvp.1);
        }

        self.tiles = new_tiles;
        *self.position_mut() = new_self_position;
        *self.size_mut() = match rotation {
            CardinalRotation::None | CardinalRotation::Full180 => *self.size(),
            CardinalRotation::Right90 | CardinalRotation::Left90 => {
                Size::new(self.size().height(), self.size().width())
            }
        }
    }

    fn tile_type_at_local(&self, pos: Position) -> Option<TileType> {
        let mut output = None;
        if !self.sub_maps.is_empty() {
            let maps = MAPS.read();
            for sub_map in &self.sub_maps {
                let map = maps[sub_map.value()].read();
                let sub_map_position = *sub_map.local_position();
                let local_position = pos - sub_map_position + *self.position();
                let test = map.tile_type_at_local(local_position);
                output = *TileTypeStandardCmp::return_greater_option(&output, &test);
            }
        }

        let self_tile_type = self.tiles.get(&(pos + *self.position())).copied();
        output = *TileTypeStandardCmp::return_greater_option(&output, &self_tile_type);

        output
    }

    fn tile_type_at_local_mut(&mut self, pos: Position) -> Option<&mut TileType> {
        self.tiles.get_mut(&(pos + *self.position()))
    }

    fn tile_type_at_local_set(&mut self, pos: Position, tile_type: TileType) -> Option<TileType> {
        if !self.sub_maps.is_empty() {
            let maps = MAPS.read();
            for sub_map in &self.sub_maps {
                let mut map = maps[sub_map.value()].write();
                let sub_map_position = *sub_map.local_position();
                let local_position = pos - sub_map_position;
                if map.is_local_position_valid(local_position) {
                    map.tile_type_at_local_set(local_position, tile_type);
                }
            }
        }

        // Since the minimum is zero, there is no sign loss to worry about.
        #[allow(clippy::cast_sign_loss)]
        {
            *self.size_mut().height_mut() = self.size().height().max(pos.y() as u32 + 1);
            *self.size_mut().width_mut() = self.size().width().max(pos.x() as u32 + 1);
        };

        self.tiles.insert(pos + *self.position(), tile_type)
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
            for sub_map in &self.sub_maps {
                let map = maps[sub_map.value()].read();
                let sub_map_position = *sub_map.local_position();
                let local_position = pos - sub_map_position + *self.position();
                let test = map.tile_type_at_local(local_position);
                output = match sort_best(&output, &test) {
                    std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => output,
                    std::cmp::Ordering::Less => test,
                };
            }
        }

        let self_tile_type = self.tiles.get(&(pos + *self.position())).copied();
        output = match sort_best(&output, &self_tile_type) {
            std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => output,
            std::cmp::Ordering::Less => self_tile_type,
        };

        output
    }
}

impl Placed for SparseMap {}

impl PlacedObject for SparseMap {}

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

impl Shape for SparseMap {
    fn box_shape_clone(&self) -> Box<dyn Shape> {
        Box::new((*self).clone())
    }

    fn iter(&self) -> ShapeIterator {
        ShapeIterator::new(self)
    }
}

impl SubMapCollection for SparseMap {
    fn add_sub_map(&mut self, local_position: Position, target: MapId) {
        let mut target_area = *MAPS.read()[target].read().area();
        let area = self.area_mut();
        *target_area.position_mut() = *target_area.position() + local_position;
        let right_pin = area.right();
        let bottom_pin = area.bottom();
        *area.position_mut().x_mut() = area.position().x().min(target_area.position().x());
        *area.position_mut().y_mut() = area.position().y().min(target_area.position().y());
        area.right_set(area.right().max(target_area.right()).max(right_pin));
        area.bottom_set(area.bottom().max(target_area.bottom()).max(bottom_pin));

        self.sub_maps.push(SubMap::new(local_position, target));
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

//! A Rust library for the pathfinder in Oldschool Runescape.

use pathfinding::prelude::bfs;
use std::collections::HashMap;

static WEST: (i32, i32) = (-1, 0);
static EAST: (i32, i32) = (1, 0);
static SOUTH: (i32, i32) = (0, -1);
static NORTH: (i32, i32) = (0, 1);
static SOUTH_WEST: (i32, i32) = (-1, -1);
static SOUTH_EAST: (i32, i32) = (1, -1);
static NORTH_WEST: (i32, i32) = (-1, 1);
static NORTH_EAST: (i32, i32) = (1, 1);

static DEFAULT_PATHFINDING_MAX_RANGE: i32 = 64;

type CollisionTile = i32;
pub struct CollisionMap {
    pub tiles: HashMap<Coordinate, CollisionTile>,
}

type Coordinate = (i32, i32, i32);

pub struct Pathfinder {}

impl Default for Pathfinder {
    fn default() -> Self {
        Self::new()
    }
}

impl Pathfinder {
    // TODO: Take the cache as input and then load collision (along with XTEA keys too)
    pub fn new() -> Pathfinder {
        Pathfinder {}
    }

    pub fn find_path(
        &self,
        start: Coordinate,
        end: Coordinate,
        collision_map: &CollisionMap,
    ) -> Option<Vec<Coordinate>> {
        bfs(
            &start,
            |p| get_successors(p, &start, collision_map),
            |p| p == &end,
        )
    }
}

fn get_successors(
    own: &Coordinate,
    start_tile: &Coordinate,
    collision_map: &CollisionMap,
) -> Vec<Coordinate> {
    let mut successors = Vec::new();

    // If the 128x128 region has been exceeded, the default pathfinding is stopped
    if own.0 > start_tile.0 + DEFAULT_PATHFINDING_MAX_RANGE
        || own.0 < start_tile.0 - DEFAULT_PATHFINDING_MAX_RANGE
        || own.1 > start_tile.1 + DEFAULT_PATHFINDING_MAX_RANGE
        || own.1 < start_tile.1 - DEFAULT_PATHFINDING_MAX_RANGE
    {
        return successors;
    }

    // Source on the order of tiles: https://oldschool.runescape.wiki/w/Pathfinding#Determining_the_target_tile
    check_successor(own, collision_map, &mut successors, WEST);
    check_successor(own, collision_map, &mut successors, EAST);
    check_successor(own, collision_map, &mut successors, SOUTH);
    check_successor(own, collision_map, &mut successors, NORTH);
    check_successor(own, collision_map, &mut successors, SOUTH_WEST);
    check_successor(own, collision_map, &mut successors, SOUTH_EAST);
    check_successor(own, collision_map, &mut successors, NORTH_WEST);
    check_successor(own, collision_map, &mut successors, NORTH_EAST);

    successors
}

fn check_successor(
    coord: &Coordinate,
    collision_map: &CollisionMap,
    successors: &mut Vec<Coordinate>,
    direction: (i32, i32),
) {
    let current_coordinate: Coordinate = (coord.0 + direction.0, coord.1 + direction.1, coord.2);

    if let Some(_blocked_tile) = collision_map.tiles.get(&current_coordinate) {
    } else {
        successors.push((
            current_coordinate.0,
            current_coordinate.1,
            current_coordinate.2,
        ));
    }
}

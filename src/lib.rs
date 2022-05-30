//! A Rust library for the pathfinder in Oldschool Runescape.

use anyhow::Result;
use pathfinding::prelude::bfs;
use rscache::{
    loader::osrs::{LocationLoader, MapLoader},
    Cache,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

static WEST: Direction = (-1, 0);
static EAST: Direction = (1, 0);
static SOUTH: Direction = (0, -1);
static NORTH: Direction = (0, 1);
static SOUTH_WEST: Direction = (-1, -1);
static SOUTH_EAST: Direction = (1, -1);
static NORTH_WEST: Direction = (-1, 1);
static NORTH_EAST: Direction = (1, 1);

static DEFAULT_PATHFINDING_MAX_RANGE: i32 = 64;

type CollisionTile = i32;
pub struct CollisionMap {
    pub tiles: HashMap<Coordinate, CollisionTile>,
}

type Coordinate = (i32, i32, i32);
type Direction = (i32, i32);

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

    pub fn from_cache(cache: &str) -> Result<Pathfinder> {
        // Load the cache
        let cache_file = Cache::new(cache)?;

        // Load all pathfinding related data
        let mut loc_loader = LocationLoader::new(&cache_file);
        let mut map_loader = MapLoader::new(&cache_file);

        /*let loc_lumbridge = loc_loader.load(12850, &[12, 12, 12, 12])?;
        let map_lumbridge = map_loader.load(12850)?;*/

        Ok(Pathfinder {})
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
    direction: Direction,
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

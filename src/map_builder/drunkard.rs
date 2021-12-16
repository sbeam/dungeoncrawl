use super::MapArchitect;
use crate::prelude::*;

const STAGGER_DISTANCE: usize = 400;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

pub struct DrunkardsWalkArchitect {}

impl MapArchitect for DrunkardsWalkArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        println!("Using DrunkardsWalkArchitect");
        let mut mb = MapBuilder::new();
        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        while mb
            .map
            .tiles
            .iter()
            .filter(|t| **t == TileType::Floor)
            .count()
            < DESIRED_FLOOR
        {
            self.drunkard(
                &Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT)),
                rng,
                &mut mb.map,
            );
            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &[mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0,
            );
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &2000.0)
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }
        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}

impl DrunkardsWalkArchitect {
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut pos = *start;
        let mut staggered = 0;

        loop {
            let idx = map.point2d_to_index(pos);
            map.tiles[idx] = TileType::Floor;

            match rng.range(0, 5) {
                0 => pos.x -= 1,
                1 => pos.x += 1,
                2 => pos.y -= 1,
                _ => pos.y += 1,
            }
            if !map.in_bounds(pos) {
                break;
            }

            staggered += 1;
            if staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}

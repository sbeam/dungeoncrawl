use super::MapArchitect;
use crate::prelude::*;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        println!("Using RoomsArchitect");
        let mut mb = MapBuilder::new();
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb.amulet_start = mb.find_most_distant();
        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }
        mb
    }
}

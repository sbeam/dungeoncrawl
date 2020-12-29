use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MonsterMovementType)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, world: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MonsterMovementType)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let player_pos = player.iter(world).next().unwrap().0;
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    movers
        .iter(world)
        .filter(|(_, _, movement)| **movement == MonsterMovementType::Chasing)
        .for_each(|(entity, pos, _)| {
            let idx = map_idx(pos.x, pos.y);
            if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
                let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
                if distance < 4.0 {
                    println!("distance to player: {:?}", distance);
                }

                let destination = if distance > 1.2 {
                    map.index_to_point2d(destination)
                } else {
                    *player_pos
                };
                if distance < 4.0 {
                    println!("dest={:?}", destination);
                }

                let mut attacked = false;
                positions
                    .iter(world)
                    .filter(|(_, target, _)| **target == destination)
                    .for_each(|(victim, _t, _)| {
                        if world
                            .entry_ref(*victim)
                            .unwrap()
                            .get_component::<Player>()
                            .is_ok()
                        {
                            println!("attack! {:?}", _t);
                            commands.push((
                                (),
                                WantsToAttack {
                                    attacker: *entity,
                                    victim: *victim,
                                },
                            ));
                            attacked = true;
                        }
                    });
                if !attacked {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: *entity,
                            destination,
                        },
                    ));
                }
            }
        })
}

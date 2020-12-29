use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MonsterMovementType)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(world: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MonsterMovementType)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();

    movers
        .iter(world)
        .filter(|(_, _, movement)| **movement == MonsterMovementType::Drunk)
        .for_each(|(entity, pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            let destination = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *pos;

            let mut attacked = false;
            positions
                .iter(world)
                .filter(|(_, pos, _)| **pos == destination)
                .for_each(|(victim, _, _)| {
                    // if the victim is the player and it's in the world and unwraps
                    if world
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                victim: *victim,
                            },
                        ));
                    }
                    attacked = true;
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
        });
}

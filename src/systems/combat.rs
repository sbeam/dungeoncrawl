use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(world: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity)> = attackers
        .iter(world)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        let is_player = world
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(mut health) = world
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            let mut rng = RandomNumberGenerator::new();

            health.current -= rng.roll_dice(1, 5);
            if health.current < 1 && !is_player {
                println!("dead!");
                commands.remove(*victim);
            } else {
                println!("health after attack = {}", health.current);
            }
        } else {
            println!("victim does not have health?");
        }
        commands.remove(*message);
    });
}

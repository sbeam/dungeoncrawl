use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(world: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity)> = attackers
        .iter(world)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        if let Ok(mut health) = world
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= 1;
            if health.current < 1 {
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

use crate::prelude::*;

#[system]
#[read_component(Health)]
pub fn end_turn(world: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let player_hp = <&Health>::query()
        .filter(component::<Player>())
        .iter(world)
        .next();

    // due to Copy impl, de-referencing trivially makes a copy here (?)
    let mut new_state = match *turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => *turn_state,
    };

    if let Some(p) = player_hp {
        if p.current < 0 {
            new_state = TurnState::GameOver;
        }
    }

    *turn_state = new_state;
}

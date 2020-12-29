use crate::prelude::*;

pub fn spawn_player(world: &mut World, pos: Point) {
    world.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
        },
    ));
}

pub fn spawn_monster(world: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph, movement) = match rng.roll_dice(1, 10) {
        1..=6 => goblin(),
        _ => orc(),
    };

    world.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
            glyph,
        },
        movement,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
    ));
}

fn goblin() -> (i32, String, FontCharType, MonsterMovementType) {
    let mut rng = RandomNumberGenerator::new();
    (
        3,
        "Goblin".to_string(),
        to_cp437('g'),
        match rng.roll_dice(1, 6) {
            1 => MonsterMovementType::Drunk,
            _ => MonsterMovementType::Chasing,
        },
    )
}

fn orc() -> (i32, String, FontCharType, MonsterMovementType) {
    (
        7,
        "Orc".to_string(),
        to_cp437('o'),
        MonsterMovementType::Chasing,
    )
}

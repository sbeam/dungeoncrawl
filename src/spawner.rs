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
            current: 90,
            max: 90,
        },
        FieldOfView::new(8),
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
        FieldOfView::new(6),
    ));
}

pub fn spawn_amulet(world: &mut World, pos: Point) {
    world.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}

fn goblin() -> (i32, String, FontCharType, MonsterMovementType) {
    let mut rng = RandomNumberGenerator::new();
    (
        1,
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
        3,
        "Orc".to_string(),
        to_cp437('o'),
        MonsterMovementType::Chasing,
    )
}

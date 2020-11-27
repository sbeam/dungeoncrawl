use crate::prelude::*;

pub fn spawn_player(world: &mut World, pos: Point) {
    world.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
            glyph: to_cp437('@'),
        },
    ));
}

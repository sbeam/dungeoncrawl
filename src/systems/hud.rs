use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(world: &SubWorld) {
    // query Healthy things and narrow to Player comps
    let mut health_query = <&Health>::query().filter(component::<Player>());
    // pop()
    let player_health = health_query.iter(world).next().unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2); // draw on layer 2

    draw_batch.print_centered(1, "Explore the dungeon. Use arrow keys to move.");

    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health {}/{}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );
    draw_batch.submit(10000).expect("Batch error");
}

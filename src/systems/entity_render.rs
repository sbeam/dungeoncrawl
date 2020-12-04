use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn entity_render(world: &mut SubWorld, #[resource] camera: &mut Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(world)
        .for_each(|pos| {
            println!("render: {:?}", pos);
            // draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}

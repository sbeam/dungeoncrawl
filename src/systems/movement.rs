use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        // replaces the Point component of the entity with the destination
        commands.add_component(want_move.entity, want_move.destination);
        if let Ok(entry) = world.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                // adds the cloned+dirty fov to the entity, replacing the existing
                commands.add_component(want_move.entity, fov.clone_dirty());
            }
            // if it's the player moving, adjust the camera
            if entry.get_component::<Player>().is_ok() {
                camera.on_player_move(want_move.destination);
            }
        }
    }
    commands.remove(*entity);
}

use crate::prelude::*;

/// Processes WantsToMove requests
#[system(for_each)] // Runs system once for each entity matching arguments
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    // Checks if desired movement destination is valid
    if map.can_enter_tile(want_move.destination) {
        // Schedules movement to desired destination
        commands.add_component(want_move.entity, want_move.destination);

        // Gets entry for entity that wants to move
        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            // Gets entry's FieldOfView component, if it has one
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                // Adds a new FieldOfView with a blank set of visible tiles and is_dirty set to true
                commands.add_component(want_move.entity, fov.clone_dirty());
                
                // Moves camera to the new destination if entry is Player
                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    });
                }
            }
        }
    }

    // Removes message so it isn't evaluated again
    commands.remove(*entity);
}

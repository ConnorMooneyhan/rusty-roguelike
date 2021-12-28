use crate::prelude::*;

/// Processes WantsToMove requests
#[system(for_each)] // Runs system once for each entity matching arguments
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if let Ok(entry) = ecs.entry_ref(want_move.entity) {
        if let Ok(fov) = entry.get_component::<FieldOfView>() {
            commands.add_component(want_move.entity, fov.clone_dirty());
        }

        if entry.get_component::<Player>().is_ok() {
            camera.on_player_move(want_move.destination);
        }
    }
    
    // Checks if desired movement destination is valid
    if map.can_enter_tile(want_move.destination) {
        // Schedules movement to desired destination
        commands.add_component(want_move.entity, want_move.destination);

        // Center camera if entity that moved is Player
        if ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(want_move.destination);
        }
    }

    // Removes message so it isn't evaluated again
    commands.remove(*entity);
}

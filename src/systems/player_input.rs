use crate::prelude::*;

/// Determines delta based on player input and, 
/// if valid, moves players and camera to new location
#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            _ => Point::zero(),
        };

        if delta != Point::zero() {
            <&mut Point>::query()
                .filter(component::<Player>())
                .iter_mut(ecs)
                .for_each(|pos| {
                    let destination = *pos + delta;
                    if map.can_enter_tile(destination) {
                        *pos = destination;
                        camera.on_player_move(destination);
                    }
                });
        }
    }
}
use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
/// Determines delta based on player input and, 
/// if valid, moves players and camera to new location
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
            let mut players = <&mut Point>::query()
                .filter(component::<Player>());
            for pos in players.iter_mut(ecs) {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            }
        }
    }
}
use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    <&mut Point>::query()
        .filter(component::<MovingRandomly>())
        .iter_mut(ecs)
        .for_each(|pos| {
            let mut rng = RandomNumberGenerator::new();
            let destination = match rng.range(0, 4) {
                0 => Point::new(-1, 0), // Left
                1 => Point::new(1, 0), // Right
                2 => Point::new(0, -1), // Up
                _ => Point::new(0, 1), // Down
            } + *pos;
            if map.can_enter_tile(destination) {
                *pos = destination;
            }
        });
}
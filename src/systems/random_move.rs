use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point)>::query().filter(component::<MovingRandomly>());
    movers.iter(ecs).for_each(|(entity, pos)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0), // Left
            1 => Point::new(1, 0),  // Right
            2 => Point::new(0, -1), // Up
            _ => Point::new(0, 1),  // Down
        } + *pos;
        commands.push((
            (),
            WantsToMove {
                entity: *entity,
                destination,
            },
        ));
    });
}

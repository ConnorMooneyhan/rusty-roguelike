use crate::prelude::*;

/// Renders map within camera dimensions
#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera, #[resource] theme: &Box<dyn MapTheme>) {
    // Gets player fov
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    // Initializes a new draw batch for the map layer
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    // Iterates through tiles within camera range
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let idx = map_idx(x, y);
            // Executes if pt is in bounds and either visible or revealed
            if map.in_bounds(pt) && player_fov.visible_tiles.contains(&pt) | map.revealed_tiles[idx]
            {
                // Tints squares based on whether they're visible or just previously revealed
                let tint = match player_fov.visible_tiles.contains(&pt) {
                    true => WHITE,
                    false => DARK_GRAY,
                };

                // Schedules tile drawing
                let glyph = theme.tile_to_render(map.tiles[idx]);
                draw_batch.set(
                    pt - offset,
                    ColorPair::new(tint, BLACK),
                    glyph,
                );
            }
        }
    }

    // Submits batch
    draw_batch.submit(0).expect("Batch error");
}

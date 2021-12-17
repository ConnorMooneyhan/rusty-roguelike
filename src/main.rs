#![warn(clippy::all, clippy::pedantic)]

mod camera;
mod map;
mod map_builder;
mod components;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Clear console layers
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        // Execute systems
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);

        // TODO: Render Draw Buffer
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Rusty Roguelike")
        .with_fps_cap(30.0) // Throttles fps to specified
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) // Dimensions of window displayed
        .with_tile_dimensions(32, 32) // How big is a character?
        .with_resource_path("resources/") // Where are my resources coming from?
        .with_font("dungeonfont.png", 32, 32) // Which resource file is my font?
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // Map layer
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // Player layer -- transparent
        .build()?;

    main_loop(context, State::new())
}

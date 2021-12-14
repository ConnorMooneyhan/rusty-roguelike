#![warn(clippy::all, clippy::pedantic)]

mod map;

mod prelude {
    use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    use crate::map::*;    
}

use prelude::*;

fn main() {
    println!("Hello there, adventurer!");
}

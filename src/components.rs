use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Render {
    color: ColorPair, // Foreground and background color
    glyph: FontCharType, // Single character or glyph
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Player;

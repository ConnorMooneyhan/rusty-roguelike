use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair, // Foreground and background color
    pub glyph: FontCharType, // Single character or glyph
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Player;

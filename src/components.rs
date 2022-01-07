pub use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Adventurer;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Enemy;
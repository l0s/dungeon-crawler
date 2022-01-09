pub use crate::prelude::*;

// Rendering

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

// Characters

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Adventurer;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Enemy;

// Properties

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MovingRandomly;

// Intents

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

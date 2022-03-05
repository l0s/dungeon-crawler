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

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Name(pub String);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Health {
    /// The remaining hit points of the character
    pub current: u32,

    /// The starting hit points of the character
    pub max: u32,
}

// States

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MovingRandomly;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ChasingAdventurer;

// Intents

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub target: Entity,
}

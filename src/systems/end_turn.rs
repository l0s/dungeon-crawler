use crate::prelude::*;

use TurnState::{AwaitingInput, MonsterTurn, PlayerTurn};

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    let new = match turn_state {
        AwaitingInput => return,
        PlayerTurn => MonsterTurn,
        MonsterTurn => AwaitingInput,
    };
    *turn_state = new;
}

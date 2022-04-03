use crate::prelude::*;

use TurnState::{AwaitingInput, MonsterTurn, PlayerTurn};

#[system]
#[read_component(Health)]
#[read_component(Adventurer)]
pub fn end_turn(ecs: &mut SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut hit_points = <&Health>::query().filter(component::<Adventurer>());
    let previous_state = *turn_state;
    let mut new_state = match previous_state {
        AwaitingInput => return,
        PlayerTurn => MonsterTurn,
        MonsterTurn => AwaitingInput,
        _ => previous_state,
    };
    hit_points.iter(ecs).for_each(|points| {
        if points.current < 1 {
            new_state = TurnState::GameOver;
        }
    });
    *turn_state = new_state;
}

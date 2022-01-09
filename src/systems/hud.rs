use crate::prelude::*;

#[system(for_each)]
#[read_component(Health)]
#[read_component(Adventurer)]
#[filter(component::<Adventurer>())]
pub fn hud(player_health: &Health) {
    let mut batch = DrawBatch::new();
    batch.target(HUD_LAYER);
    batch.print_centered(1, "Explore the Dungeon. Arrow keys to move");
    batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );
    batch.submit(16_384).expect("Error rendering HUD");
}

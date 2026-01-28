use bevy::prelude::*;
use crate::components::character::Player;
use crate::systems::chain::ChainState;

/// Combo hit flash with escalating colors
///
/// Draws colored rings around characters based on combo count:
/// - 1 hit: White
/// - 2 hits: Yellow
/// - 3 hits: Orange
/// - 4+ hits: Red-orange
pub fn combo_hit_flash(
    chain_query: Query<(&ChainState, &Transform, &Player), Changed<ChainState>>,
    mut gizmos: Gizmos,
) {
    for (chain_state, transform, player) in chain_query.iter() {
        if chain_state.hit_count == 0 {
            continue;
        }

        let (color, radius) = match chain_state.hit_count {
            1 => (Color::srgba(1.0, 1.0, 1.0, 0.8), 35.0),     // White
            2 => (Color::srgba(1.0, 1.0, 0.0, 0.9), 45.0),     // Yellow
            3 => (Color::srgba(1.0, 0.65, 0.0, 1.0), 55.0),    // Orange
            _ => (Color::srgba(1.0, 0.3, 0.0, 1.0), 65.0),     // Red-orange
        };

        // Draw flash ring around character
        let pos = transform.translation.truncate();
        gizmos.circle_2d(pos, radius, color);

        trace!("Hit flash: Player {:?} - {} hits ({:?})", player, chain_state.hit_count, color);
    }
}

/// Debug: Log combo hits for visibility
pub fn debug_combo_hits(
    query: Query<(&Player, &ChainState), Changed<ChainState>>,
) {
    for (player, chain) in query.iter() {
        if chain.hit_count > 0 {
            info!("Player {:?}: {} HIT COMBO!", player, chain.hit_count);
        }
    }
}

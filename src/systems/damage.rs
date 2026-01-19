use bevy::prelude::*;
use crate::components::character::Player;
use crate::components::state::CharacterState;
use crate::events::combat_events::HitEvent;

/// Apply hit reactions when attacks connect
pub fn apply_hit_reactions(
    mut hit_events: EventReader<HitEvent>,
    mut query: Query<(&mut CharacterState, &mut Sprite, &Player)>,
) {
    for event in hit_events.read() {
        // Skip if the hit was blocked (guard system handles that)
        if event.was_blocked {
            continue;
        }

        // Apply hitstun to defender
        if let Ok((mut state, mut sprite, player)) = query.get_mut(event.defender) {
            // Enter stagger state (brief hitstun)
            let hitstun_frames = match event.damage {
                1 => 15, // Light attack: 15 frames (~0.25 seconds)
                2 => 25, // Heavy attack: 25 frames (~0.42 seconds)
                _ => 20, // Default
            };

            *state = CharacterState::Staggered {
                frames_remaining: hitstun_frames,
            };

            // Visual feedback: flash red briefly
            sprite.color = Color::srgb(1.0, 0.3, 0.3);

            info!(
                "HIT! Player {:?} took {} damage ({} frames hitstun)",
                player, event.damage, hitstun_frames
            );
        }
    }
}

/// Visual feedback for successful hits (flash effect)
pub fn hit_flash_feedback(
    mut query: Query<(&CharacterState, &mut Sprite, &Player), Changed<CharacterState>>,
) {
    for (state, mut sprite, player) in query.iter_mut() {
        // Flash red when entering hitstun
        if matches!(state, CharacterState::Staggered { .. }) {
            sprite.color = Color::srgb(1.0, 0.3, 0.3);
        }
    }
}

/// Debug: Log all hit events
pub fn debug_hit_events(
    mut hit_events: EventReader<HitEvent>,
) {
    for event in hit_events.read() {
        debug!(
            "HitEvent: attacker={:?}, defender={:?}, damage={}, blocked={}",
            event.attacker, event.defender, event.damage, event.was_blocked
        );
    }
}

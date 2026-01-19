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
            // Base hitstun
            let base_hitstun = match event.damage {
                1 => 15, // Light attack: 15 frames (~0.25 seconds)
                2 => 25, // Heavy attack: 25 frames (~0.42 seconds)
                _ => 20, // Default
            };

            // Counter hit bonus: +10 frames
            let hitstun_frames = if event.counter_hit {
                base_hitstun + 10
            } else {
                base_hitstun
            };

            *state = CharacterState::Staggered {
                frames_remaining: hitstun_frames,
            };

            // Visual feedback: gold for counter hit, red for normal hit
            sprite.color = if event.counter_hit {
                Color::srgb(1.0, 0.85, 0.0) // Gold/yellow for counter hit
            } else {
                Color::srgb(1.0, 0.3, 0.3) // Red for normal hit
            };

            if event.counter_hit {
                info!(
                    "COUNTER HIT! Player {:?} took {} damage ({} frames hitstun)",
                    player, event.damage, hitstun_frames
                );
            } else {
                info!(
                    "HIT! Player {:?} took {} damage ({} frames hitstun)",
                    player, event.damage, hitstun_frames
                );
            }
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

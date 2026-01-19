use bevy::prelude::*;
use crate::components::character::{Player, Velocity};
use crate::components::state::{CharacterState, StateTimer};
use crate::systems::input::CurrentInputs;

/// Evade state component to track i-frame window
#[derive(Component, Debug)]
pub struct EvadeData {
    pub direction: Vec2,
    pub invincible: bool,
}

/// Handle evade input (Shift + direction)
pub fn handle_evade_input(
    mut commands: Commands,
    inputs: Res<CurrentInputs>,
    mut query: Query<(Entity, &Player, &mut CharacterState, &mut Velocity)>,
) {
    for (entity, player, mut state, mut velocity) in query.iter_mut() {
        // Can only evade from Idle or Walking states
        if !matches!(*state, CharacterState::Idle | CharacterState::Walking) {
            continue;
        }

        // Get input for this player
        let input = match player {
            Player::One => &inputs.player_one,
            Player::Two => &inputs.player_two,
        };

        // Check for evade input (step = Shift + direction)
        if input.step && input.movement.length() > 0.1 {
            let direction = input.movement.normalize();

            // Enter evade state
            *state = CharacterState::Idle; // We'll use a marker component instead

            // Add evade data component
            commands.entity(entity).insert((
                EvadeData {
                    direction,
                    invincible: false, // Becomes true after startup
                },
                StateTimer::new(15), // Total evade duration: 3f startup + 4f active + 8f recovery
            ));

            // Apply evade movement (fast dash)
            velocity.0 = direction * 500.0; // Fast movement

            info!("Player {:?} evading in direction {:?}", player, direction);
        }
    }
}

/// Progress evade state and manage i-frames
pub fn progress_evade(
    mut commands: Commands,
    mut query: Query<(Entity, &mut EvadeData, &mut StateTimer, &mut Velocity)>,
) {
    for (entity, mut evade, mut timer, mut velocity) in query.iter_mut() {
        timer.tick();

        // I-frames: active from frame 3 to frame 7 (4 frames total)
        if timer.elapsed >= 3 && timer.elapsed < 7 {
            evade.invincible = true;
        } else {
            evade.invincible = false;
        }

        // Slow down during recovery (after frame 7)
        if timer.elapsed >= 7 {
            velocity.0 *= 0.8; // Decelerate
        }

        // End evade when timer completes
        if timer.is_complete() {
            commands.entity(entity).remove::<EvadeData>();
            commands.entity(entity).remove::<StateTimer>();
            velocity.0 = Vec2::ZERO; // Stop
            debug!("Evade complete");
        }
    }
}

/// Make evading characters invincible during i-frames
/// This prevents HitEvents from being processed
pub fn evade_invincibility(
    evade_query: Query<(Entity, &EvadeData)>,
    mut hit_events: EventReader<crate::events::combat_events::HitEvent>,
) {
    use crate::events::combat_events::HitEvent;

    // Check if any hit event targets an invincible evading character
    for event in hit_events.read() {
        if let Ok((_, evade)) = evade_query.get(event.defender) {
            if evade.invincible {
                // Hit was evaded! (event is consumed but not processed)
                info!("Attack evaded! I-frames active");
            }
        }
    }
}

/// Visual feedback for evading (motion blur effect)
pub fn visualize_evade(
    mut query: Query<(&EvadeData, &mut Sprite, &Player)>,
) {
    for (evade, mut sprite, player) in query.iter_mut() {
        // Base colors
        let base_color = match player {
            Player::One => Color::srgb(0.9, 0.2, 0.2),   // Red
            Player::Two => Color::srgb(0.2, 0.4, 0.9),   // Blue
        };

        if evade.invincible {
            // Semi-transparent during i-frames (ghostly)
            sprite.color = Color::srgba(
                base_color.to_srgba().red,
                base_color.to_srgba().green,
                base_color.to_srgba().blue,
                0.5, // 50% transparency
            );
        } else {
            // Slight transparency during startup/recovery
            sprite.color = Color::srgba(
                base_color.to_srgba().red,
                base_color.to_srgba().green,
                base_color.to_srgba().blue,
                0.7, // 70% transparency
            );
        }
    }
}

/// Clean up sprite when evade ends
pub fn cleanup_evade_visuals(
    mut removed: RemovedComponents<EvadeData>,
    mut query: Query<(&mut Sprite, &Player)>,
) {
    for entity in removed.read() {
        if let Ok((mut sprite, player)) = query.get_mut(entity) {
            // Restore full opacity
            let base_color = match player {
                Player::One => Color::srgb(0.9, 0.2, 0.2),
                Player::Two => Color::srgb(0.2, 0.4, 0.9),
            };
            sprite.color = base_color;
        }
    }
}

use bevy::prelude::*;
use crate::components::character::Player;
use crate::components::guard::GuardMeter;
use crate::components::state::{CharacterState, StateTimer};
use crate::events::combat_events::{GuardBreakEvent, HitEvent, ParryEvent};
use crate::systems::input::CurrentInputs;

/// Handle block/parry input - hold for block, tap for parry
pub fn handle_block_input(
    mut commands: Commands,
    inputs: Res<CurrentInputs>,
    mut query: Query<(Entity, &Player, &mut CharacterState)>,
) {
    for (entity, player, mut state) in query.iter_mut() {
        // Can only block/parry from Idle or Walking states
        let can_block = matches!(*state, CharacterState::Idle | CharacterState::Walking);

        if !can_block {
            continue;
        }

        // Get input for this player
        let input = match player {
            Player::One => &inputs.player_one,
            Player::Two => &inputs.player_two,
        };

        // Check for parry attempt (tap block - just_pressed would be detected in input.rs)
        // For now, we'll use a simple heuristic: if block is pressed while in Idle/Walking,
        // and it's a new press, initiate parry. If held, transition to blocking.

        // Note: The input system tracks 'block' as pressed state. We need just_pressed.
        // For parry vs block distinction, we'll need to track press duration.
        // Simple approach: always start with parry attempt, timeout to block if held.

        if input.block {
            match *state {
                CharacterState::Idle | CharacterState::Walking => {
                    // Start parry attempt (2f startup, 6f active window)
                    *state = CharacterState::Parrying { frames_remaining: 6 };
                    commands.entity(entity).insert(StateTimer::new(2)); // 2f startup
                    info!("Player {:?} attempting parry", player);
                }
                CharacterState::Parrying { .. } => {
                    // Already parrying, do nothing
                }
                _ => {}
            }
        } else {
            // Block released
            if matches!(*state, CharacterState::Blocking) {
                *state = CharacterState::Idle;
                debug!("Player {:?} stopped blocking", player);
            }
        }
    }
}

/// Fill guard meter when blocking attacks
pub fn fill_guard_on_block(
    mut hit_events: EventReader<HitEvent>,
    mut guard_query: Query<(&mut GuardMeter, &CharacterState)>,
) {
    for event in hit_events.read() {
        // Check if defender is blocking
        if let Ok((mut guard, state)) = guard_query.get_mut(event.defender) {
            if *state == CharacterState::Blocking && !event.unblockable {
                // Calculate guard damage based on attack damage
                let guard_damage = match event.damage {
                    1 => 0.15, // Light attack: +15% guard
                    2 => 0.35, // Heavy attack: +35% guard
                    _ => 0.10, // Default
                };

                guard.fill(guard_damage);

                info!(
                    "Guard meter filled by {:.0}% (now at {:.0}%)",
                    guard_damage * 100.0,
                    guard.current * 100.0
                );
            }
        }
    }
}

/// Passively drain guard meter when not blocking
pub fn drain_guard_meter(
    time: Res<Time>,
    mut query: Query<(&mut GuardMeter, &CharacterState)>,
) {
    for (mut guard, state) in query.iter_mut() {
        // Only drain when not blocking and guard > 0
        if *state != CharacterState::Blocking && guard.current > 0.0 {
            // Drain 5% per second
            let drain_rate = 0.05 * time.delta_seconds();
            guard.drain(drain_rate);
        }
    }
}

/// Check for guard break and trigger stagger
pub fn check_guard_break(
    mut query: Query<(Entity, &mut GuardMeter, &mut CharacterState), Changed<GuardMeter>>,
    mut break_events: EventWriter<GuardBreakEvent>,
) {
    for (entity, mut guard, mut state) in query.iter_mut() {
        if guard.is_broken() {
            // Guard broken! Enter stagger state
            *state = CharacterState::Staggered {
                frames_remaining: 40, // ~0.67 seconds at 60 FPS
            };

            guard.reset();

            break_events.send(GuardBreakEvent { entity });

            warn!("Guard broken! Entity {:?} is staggered", entity);
        }
    }
}

/// Progress stagger state (count down frames)
pub fn progress_stagger(
    mut query: Query<&mut CharacterState>,
) {
    for mut state in query.iter_mut() {
        if let CharacterState::Staggered { frames_remaining } = &mut *state {
            if *frames_remaining > 0 {
                *frames_remaining -= 1;
            } else {
                // Stagger complete, return to idle
                *state = CharacterState::Idle;
                debug!("Stagger complete, returning to Idle");
            }
        }
    }
}

/// Progress parry window (startup then active window)
pub fn progress_parry(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CharacterState, Option<&mut StateTimer>)>,
) {
    for (entity, mut state, timer) in query.iter_mut() {
        if let CharacterState::Parrying { frames_remaining } = &mut *state {
            // Tick down active window
            if *frames_remaining > 0 {
                *frames_remaining -= 1;
            } else {
                // Parry window expired, failed parry
                *state = CharacterState::Idle;
                commands.entity(entity).remove::<StateTimer>();
                debug!("Parry window expired, returning to Idle");
            }
        }
    }
}

/// Check for successful parries when attacks hit during parry window
pub fn check_parry_success(
    mut hit_events: EventReader<HitEvent>,
    mut query: Query<(&mut CharacterState, &mut GuardMeter, &Player)>,
    mut parry_events: EventWriter<ParryEvent>,
    mut commands: Commands,
) {
    for event in hit_events.read() {
        if let Ok((mut state, mut guard, player)) = query.get_mut(event.defender) {
            // Check if defender is in parry window
            if matches!(*state, CharacterState::Parrying { .. }) {
                // PARRY SUCCESS!
                info!("PARRY! Player {:?} deflected attack", player);

                // Send parry event
                parry_events.send(ParryEvent {
                    defender: event.defender,
                    attacker: event.attacker,
                });

                // Restore guard meter
                guard.current = (guard.current - 0.25).max(0.0);

                // Return to idle (can act immediately)
                *state = CharacterState::Idle;
                commands.entity(event.defender).remove::<StateTimer>();

                // Stagger the attacker (punish for being parried)
                if let Ok((mut attacker_state, _, _)) = query.get_mut(event.attacker) {
                    *attacker_state = CharacterState::Staggered {
                        frames_remaining: 20, // Longer stagger than normal hit
                    };
                }
            }
        }
    }
}

/// Visual feedback for blocking/parrying (change color slightly)
pub fn visualize_blocking(
    mut query: Query<(&CharacterState, &mut Sprite, &Player), Changed<CharacterState>>,
) {
    for (state, mut sprite, player) in query.iter_mut() {
        // Base colors
        let base_color = match player {
            Player::One => Color::srgb(0.9, 0.2, 0.2),   // Red
            Player::Two => Color::srgb(0.2, 0.4, 0.9),   // Blue
        };

        match state {
            CharacterState::Blocking => {
                // Silver/white when blocking
                sprite.color = Color::srgb(0.85, 0.85, 0.9);  // Bright silver
            }
            CharacterState::Parrying { .. } => {
                // Bright cyan when parrying (high risk/reward)
                sprite.color = Color::srgb(0.3, 1.0, 1.0);
            }
            CharacterState::Staggered { .. } => {
                // Gray when staggered
                sprite.color = Color::srgb(0.5, 0.5, 0.5);
            }
            _ => {
                // Return to base color (but don't override attack visualization)
                if !matches!(state, CharacterState::Attacking { .. }) {
                    sprite.color = base_color;
                }
            }
        }
    }
}

/// Visual feedback for successful parry (brief flash)
pub fn parry_flash_effect(
    mut parry_events: EventReader<ParryEvent>,
    mut query: Query<&mut Sprite>,
) {
    for event in parry_events.read() {
        // Flash defender bright white
        if let Ok(mut sprite) = query.get_mut(event.defender) {
            sprite.color = Color::srgb(1.0, 1.0, 1.0);
        }
    }
}

/// Debug: Display guard meter values
pub fn debug_guard_meter(
    query: Query<(&Player, &GuardMeter), Changed<GuardMeter>>,
) {
    for (player, guard) in query.iter() {
        debug!(
            "Player {:?} guard: {:.0}% / {:.0}%",
            player,
            guard.current * 100.0,
            guard.max * 100.0
        );
    }
}

use bevy::prelude::*;
use crate::components::character::Player;
use crate::components::movelist::AttackDirection;
use crate::components::state::{AttackPhase, AttackType, CharacterState, StateTimer};
use crate::events::combat_events::HitEvent;
use crate::systems::input::CurrentInputs;

/// Component to track chain attack state
#[derive(Component, Debug)]
pub struct ChainState {
    /// Number of attacks in current chain (0-2)
    pub chain_count: u8,
    /// Whether the last attack hit (can chain)
    pub can_chain: bool,
    /// Whether we're in the chain window
    pub in_chain_window: bool,
}

impl ChainState {
    pub fn new() -> Self {
        Self {
            chain_count: 0,
            can_chain: false,
            in_chain_window: false,
        }
    }

    pub fn reset(&mut self) {
        self.chain_count = 0;
        self.can_chain = false;
        self.in_chain_window = false;
    }

    pub fn can_continue_chain(&self) -> bool {
        self.can_chain && self.in_chain_window && self.chain_count < 2
    }
}

impl Default for ChainState {
    fn default() -> Self {
        Self::new()
    }
}

/// Mark attacks as chainable when they hit
pub fn mark_chainable_on_hit(
    mut hit_events: EventReader<HitEvent>,
    mut query: Query<(&mut ChainState, &CharacterState)>,
) {
    for event in hit_events.read() {
        // Only unblocked hits can be chained
        if event.was_blocked {
            continue;
        }

        // Check if attacker is in a Light attack
        if let Ok((mut chain_state, state)) = query.get_mut(event.attacker) {
            if let CharacterState::Attacking { attack_type, phase, .. } = state {
                if *attack_type == AttackType::Light && *phase == AttackPhase::Active {
                    // Hit landed! Can chain
                    chain_state.can_chain = true;
                    chain_state.in_chain_window = true;
                    info!("Hit landed! Chain enabled (count: {})", chain_state.chain_count);
                }
            }
        }
    }
}

/// Open chain window during recovery phase
pub fn manage_chain_window(
    mut query: Query<(&mut ChainState, &CharacterState, Option<&StateTimer>)>,
) {
    for (mut chain_state, state, timer) in query.iter_mut() {
        match state {
            CharacterState::Attacking { attack_type, phase, .. } => {
                if *attack_type == AttackType::Light && *phase == AttackPhase::Recovery {
                    // In recovery phase - check if in chain window (frames 0-7 of 10f recovery)
                    if let Some(timer) = timer {
                        let recovery_elapsed = timer.elapsed.saturating_sub(8); // 6f startup + 2f active = 8f before recovery
                        if recovery_elapsed < 7 && chain_state.can_chain {
                            chain_state.in_chain_window = true;
                        } else {
                            chain_state.in_chain_window = false;
                        }
                    }
                } else {
                    // Not in Light recovery
                    chain_state.in_chain_window = false;
                }
            }
            CharacterState::Idle => {
                // Reset chain when returning to idle
                if chain_state.chain_count > 0 {
                    debug!("Chain ended, count was: {}", chain_state.chain_count);
                    chain_state.reset();
                }
            }
            _ => {
                chain_state.in_chain_window = false;
            }
        }
    }
}

/// Handle chain attack input - cancel recovery into new Light
pub fn handle_chain_input(
    mut commands: Commands,
    inputs: Res<CurrentInputs>,
    mut query: Query<(Entity, &Player, &mut CharacterState, &mut ChainState, Option<&mut StateTimer>)>,
) {
    for (entity, player, mut state, mut chain_state, timer) in query.iter_mut() {
        // Skip if no timer (shouldn't happen during chain windows, but be safe)
        let Some(mut timer) = timer else {
            continue;
        };
        // Check if can chain
        if !chain_state.can_continue_chain() {
            continue;
        }

        // Get input for this player
        let input = match player {
            Player::One => &inputs.player_one,
            Player::Two => &inputs.player_two,
        };

        // Check for Light attack input during chain window
        if input.light_attack {
            // CHAIN CANCEL!
            chain_state.chain_count += 1;
            chain_state.can_chain = false;
            chain_state.in_chain_window = false;

            // Cancel into new Light attack
            *state = CharacterState::Attacking {
                attack_type: AttackType::Light,
                direction: AttackDirection::Neutral,
                phase: AttackPhase::Startup,
            };

            // Reset timer for new attack startup (5f - reduced for responsiveness)
            timer.reset(5);

            info!(
                "Player {:?} CHAIN CANCEL! Chain count: {}",
                player, chain_state.chain_count
            );

            // Re-add hitbox for the new attack
            let hitbox = crate::systems::attack::create_hitbox(AttackType::Light);
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.insert(hitbox);
            }
        }
    }
}

/// Visual feedback for chain window (slight sparkle/outline)
pub fn visualize_chain_window(
    mut gizmos: Gizmos,
    query: Query<(&ChainState, &Transform, &Player)>,
) {
    for (chain_state, transform, _player) in query.iter() {
        if chain_state.in_chain_window && chain_state.can_chain {
            // Draw a pulsing circle around character to indicate chain window
            let pos = transform.translation.truncate();
            let radius = 40.0;

            // Pulsing effect (would need time for actual pulse, this is static)
            gizmos.circle_2d(
                pos,
                radius,
                Color::srgb(1.0, 1.0, 0.0), // Yellow for chain window
            );
        }
    }
}

/// Debug: Log chain state changes
pub fn debug_chain_state(
    query: Query<(&Player, &ChainState), Changed<ChainState>>,
) {
    for (player, chain_state) in query.iter() {
        if chain_state.can_chain || chain_state.chain_count > 0 {
            debug!(
                "Player {:?} chain: count={}, can_chain={}, in_window={}",
                player, chain_state.chain_count, chain_state.can_chain, chain_state.in_chain_window
            );
        }
    }
}

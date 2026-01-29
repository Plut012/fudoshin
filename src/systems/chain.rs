use bevy::prelude::*;
use crate::components::character::Player;
use crate::components::combo::InputBuffer;
use crate::components::movelist::AttackDirection;
use crate::components::state::{AttackPhase, AttackType, CharacterState, StateTimer};
use crate::events::combat_events::HitEvent;
use crate::systems::input::CurrentInputs;

// ==================== INPUT BUFFER SYSTEMS ====================

/// Record button presses into input buffer each frame
///
/// This system captures all attack button presses and stores them in the
/// InputBuffer component. Inputs are held for 8 frames, making combo
/// execution feel responsive and forgiving.
pub fn record_inputs_to_buffer(
    inputs: Res<CurrentInputs>,
    mut query: Query<(&Player, &mut InputBuffer)>,
) {
    for (player, mut buffer) in query.iter_mut() {
        let input = match player {
            Player::One => &inputs.player_one,
            Player::Two => &inputs.player_two,
        };

        // Record any button presses this frame
        if input.light_attack {
            buffer.press(AttackType::Light);
            trace!("Player {:?} buffered Light input", player);
        }
        if input.heavy_attack {
            buffer.press(AttackType::Heavy);
            trace!("Player {:?} buffered Heavy input", player);
        }
        if input.grab {
            buffer.press(AttackType::Grab);
            trace!("Player {:?} buffered Grab input", player);
        }
    }
}

/// Age all input buffers by 1 frame
///
/// This system ticks down all active input buffers. Inputs older than
/// 8 frames are automatically cleared.
pub fn age_input_buffers(
    mut query: Query<&mut InputBuffer>,
) {
    for mut buffer in query.iter_mut() {
        buffer.tick();
    }
}

// ==================== CHAIN STATE ====================

/// Component to track chain attack state
#[derive(Component, Debug)]
pub struct ChainState {
    /// Number of attacks in current chain (0-2)
    pub chain_count: u8,
    /// Total hits landed (for visual escalation)
    pub hit_count: u8,
    /// Whether the last attack hit (can chain)
    pub can_chain: bool,
    /// Whether we're in the chain window
    pub in_chain_window: bool,
    /// What attack types can cancel into
    pub cancellable_into: Vec<AttackType>,
}

impl ChainState {
    pub fn new() -> Self {
        Self {
            chain_count: 0,
            hit_count: 0,
            can_chain: false,
            in_chain_window: false,
            cancellable_into: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.chain_count = 0;
        self.hit_count = 0;
        self.can_chain = false;
        self.in_chain_window = false;
        self.cancellable_into.clear();
    }

    pub fn can_continue_chain(&self) -> bool {
        self.can_chain && self.in_chain_window && self.chain_count < 2
    }

    pub fn can_cancel_into(&self, attack_type: AttackType) -> bool {
        self.can_chain
            && self.in_chain_window
            && self.cancellable_into.contains(&attack_type)
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
    mut query: Query<(&mut ChainState, &CharacterState, &crate::components::movelist::Movelist)>,
) {
    for event in hit_events.read() {
        // Only unblocked hits can be chained
        if event.was_blocked {
            continue;
        }

        // Check if attacker can chain
        if let Ok((mut chain_state, state, movelist)) = query.get_mut(event.attacker) {
            if let CharacterState::Attacking { attack_type, direction, phase } = state {
                if *phase == AttackPhase::Active {
                    // Get move data to check if it's cancellable
                    if let Some(move_data) = movelist.get_move(*attack_type, *direction) {
                        // Check if this move has any cancel options
                        if !move_data.cancellable_into.is_empty() && move_data.cancel_window_frames > 0 {
                            // Hit landed! Can chain
                            chain_state.can_chain = true;
                            chain_state.in_chain_window = true;
                            chain_state.hit_count += 1;
                            chain_state.cancellable_into = move_data.cancellable_into.clone();

                            let cancel_list: Vec<String> = move_data.cancellable_into.iter()
                                .map(|t| format!("{:?}", t))
                                .collect();

                            info!(
                                "{:?} {:?} hit landed! Chain enabled (chain: {}, hit: {}) - Can cancel into: {}",
                                attack_type,
                                direction,
                                chain_state.chain_count,
                                chain_state.hit_count,
                                cancel_list.join("/")
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Open chain window during recovery phase
pub fn manage_chain_window(
    mut query: Query<(&mut ChainState, &CharacterState, Option<&StateTimer>, &crate::components::movelist::Movelist)>,
) {
    for (mut chain_state, state, timer, movelist) in query.iter_mut() {
        match state {
            CharacterState::Attacking { attack_type, direction, phase } => {
                if *phase == AttackPhase::Recovery {
                    // Get move data to check cancel window
                    if let Some(move_data) = movelist.get_move(*attack_type, *direction) {
                        if let Some(timer) = timer {
                            // Calculate elapsed frames in recovery phase
                            let pre_recovery_frames = move_data.startup_frames + move_data.active_frames;
                            let recovery_elapsed = timer.elapsed.saturating_sub(pre_recovery_frames);

                            // Check if within cancel window
                            if recovery_elapsed < move_data.cancel_window_frames && chain_state.can_chain {
                                chain_state.in_chain_window = true;
                            } else {
                                chain_state.in_chain_window = false;
                            }
                        }
                    } else {
                        chain_state.in_chain_window = false;
                    }
                } else {
                    // Not in recovery phase
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

/// Handle chain attack input - cancel recovery into new attack
pub fn handle_chain_input(
    mut commands: Commands,
    inputs: Res<CurrentInputs>,
    mut query: Query<(
        Entity,
        &Player,
        &Transform,
        &mut CharacterState,
        &mut ChainState,
        &InputBuffer,
        Option<&mut StateTimer>
    )>,
    all_chars_query: Query<(&Transform, &Player), With<crate::components::character::Character>>,
) {
    for (entity, player, transform, mut state, mut chain_state, buffer, timer) in query.iter_mut() {
        // Skip if no timer (shouldn't happen during chain windows, but be safe)
        let Some(mut timer) = timer else {
            continue;
        };

        // Check if can chain
        if !chain_state.can_chain || !chain_state.in_chain_window {
            continue;
        }

        // Check each attack type in priority order
        // Priority: Heavy > Grab > Light (Heavy = commitment, should be intentional)
        let cancel_type = if buffer.is_buffered(AttackType::Heavy)
            && chain_state.can_cancel_into(AttackType::Heavy)
        {
            Some(AttackType::Heavy)
        } else if buffer.is_buffered(AttackType::Grab)
            && chain_state.can_cancel_into(AttackType::Grab)
        {
            Some(AttackType::Grab)
        } else if buffer.is_buffered(AttackType::Light)
            && chain_state.can_cancel_into(AttackType::Light)
        {
            Some(AttackType::Light)
        } else {
            None
        };

        if let Some(attack_type) = cancel_type {
            // Get current input for directional cancels
            let input = match player {
                Player::One => &inputs.player_one,
                Player::Two => &inputs.player_two,
            };

            // Get opponent position to determine facing direction
            let opponent_x = all_chars_query.iter()
                .find(|(_, p)| **p != *player)  // Find the other player
                .map(|(t, _)| t.translation.x)
                .unwrap_or(0.0);
            let player_x = transform.translation.x;

            // Determine attack direction from current input
            let direction = input.get_attack_direction(*player, opponent_x, player_x);

            // CHAIN CANCEL!
            info!(
                "Player {:?} CANCEL → {:?} {:?} (chain: {}, hit: {})",
                player, direction, attack_type, chain_state.chain_count, chain_state.hit_count
            );

            // Update chain state
            chain_state.chain_count += 1;
            chain_state.can_chain = false;
            chain_state.in_chain_window = false;
            chain_state.cancellable_into.clear();

            // Transition to new attack with direction
            *state = CharacterState::Attacking {
                attack_type,
                direction,  // Use player's current directional input!
                phase: AttackPhase::Startup,
            };

            // Reset timer with appropriate startup frames
            let startup = match attack_type {
                AttackType::Light => 5,
                AttackType::Heavy => 11,
                AttackType::Grab => 10,
            };
            timer.reset(startup);

            // Spawn hitbox for new attack
            let hitbox = crate::systems::attack::create_hitbox(attack_type);
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

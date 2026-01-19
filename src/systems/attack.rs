use bevy::prelude::*;
use crate::components::character::Player;
use crate::components::combat::{AttackProperty, Hitbox};
use crate::components::state::*;
use crate::systems::input::CurrentInputs;

/// Progress attack animations through phases (Startup → Active → Recovery → Idle)
pub fn progress_attack_phases(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CharacterState, &mut StateTimer)>,
) {
    for (entity, mut state, mut timer) in query.iter_mut() {
        if let CharacterState::Attacking { attack_type, phase } = *state {
            timer.tick();

            if timer.is_complete() {
                // Transition to next phase
                match phase {
                    AttackPhase::Startup => {
                        // Startup complete → Enter Active phase
                        let attack_data = get_attack_data(attack_type);
                        *state = CharacterState::Attacking {
                            attack_type,
                            phase: AttackPhase::Active,
                        };
                        timer.reset(attack_data.active_frames);

                        debug!(
                            "Attack phase: Startup → Active ({} frames)",
                            attack_data.active_frames
                        );
                    }
                    AttackPhase::Active => {
                        // Active complete → Enter Recovery phase
                        let attack_data = get_attack_data(attack_type);
                        *state = CharacterState::Attacking {
                            attack_type,
                            phase: AttackPhase::Recovery,
                        };
                        timer.reset(attack_data.recovery_frames);

                        debug!(
                            "Attack phase: Active → Recovery ({} frames)",
                            attack_data.recovery_frames
                        );
                    }
                    AttackPhase::Recovery => {
                        // Recovery complete → Return to Idle
                        *state = CharacterState::Idle;
                        commands.entity(entity).remove::<StateTimer>();

                        debug!("Attack phase: Recovery → Idle");
                    }
                }
            }
        }
    }
}

/// Activate hitboxes when entering Active phase
pub fn activate_hitboxes(
    mut query: Query<(&CharacterState, &mut Hitbox), Changed<CharacterState>>,
) {
    for (state, mut hitbox) in query.iter_mut() {
        match state {
            CharacterState::Attacking {
                phase: AttackPhase::Active,
                ..
            } => {
                hitbox.activate();
                debug!("Hitbox activated");
            }
            _ => {
                // Deactivate hitbox in all other states
                if hitbox.active {
                    hitbox.deactivate();
                    debug!("Hitbox deactivated");
                }
            }
        }
    }
}

/// Visual feedback for attack states (change color during phases)
pub fn visualize_attack_phases(
    mut query: Query<(&CharacterState, &mut Sprite, &Player)>,
) {
    for (state, mut sprite, player) in query.iter_mut() {
        // Base colors
        let base_color = match player {
            Player::One => Color::srgb(0.9, 0.2, 0.2),   // Red
            Player::Two => Color::srgb(0.2, 0.4, 0.9),   // Blue
        };

        match state {
            CharacterState::Attacking { phase, .. } => {
                sprite.color = match phase {
                    AttackPhase::Startup => {
                        // Slightly dimmed during startup
                        Color::srgb(
                            base_color.to_srgba().red * 0.7,
                            base_color.to_srgba().green * 0.7,
                            base_color.to_srgba().blue * 0.7,
                        )
                    }
                    AttackPhase::Active => {
                        // Bright white flash during active frames
                        Color::srgb(1.0, 1.0, 1.0)
                    }
                    AttackPhase::Recovery => {
                        // Return to base color
                        base_color
                    }
                };
            }
            _ => {
                // Return to base color when not attacking
                sprite.color = base_color;
            }
        }
    }
}

/// Helper function to get attack data for an attack type
fn get_attack_data(attack_type: AttackType) -> AttackData {
    match attack_type {
        AttackType::Light => AttackData::light(),
        AttackType::Heavy => AttackData::heavy(),
        AttackType::Grab => AttackData::grab(),
    }
}

/// Handle attack button inputs and initiate attacks
pub fn handle_attack_input(
    mut commands: Commands,
    inputs: Res<CurrentInputs>,
    mut query: Query<(Entity, &Player, &mut CharacterState)>,
) {
    for (entity, player, mut state) in query.iter_mut() {
        // Can only attack from Idle state
        if *state != CharacterState::Idle {
            continue;
        }

        // Get input for this player
        let input = match player {
            Player::One => &inputs.player_one,
            Player::Two => &inputs.player_two,
        };

        // Check for attack inputs
        let attack_type = if input.light_attack {
            Some(AttackType::Light)
        } else if input.heavy_attack {
            Some(AttackType::Heavy)
        } else if input.grab {
            Some(AttackType::Grab)
        } else {
            None
        };

        // Initiate attack if button pressed
        if let Some(attack_type) = attack_type {
            let attack_data = get_attack_data(attack_type);

            // Enter Attacking state
            *state = CharacterState::Attacking {
                attack_type,
                phase: AttackPhase::Startup,
            };

            // Add timer for startup phase
            commands.entity(entity).insert(StateTimer::new(attack_data.startup_frames));

            // Add/update hitbox component
            let hitbox = create_hitbox(attack_type);
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.insert(hitbox);
            }

            info!(
                "Player {:?} initiated {:?} attack (startup: {}f)",
                player, attack_type, attack_data.startup_frames
            );
        }
    }
}

/// Create a hitbox for the given attack type
fn create_hitbox(attack_type: AttackType) -> Hitbox {
    match attack_type {
        AttackType::Light => {
            // Light attack: small, fast hitbox in front of character
            let rect = Rect::from_center_size(
                Vec2::new(50.0, 0.0),  // 50 pixels in front
                Vec2::new(40.0, 60.0), // Width x Height
            );
            Hitbox::new(rect, 1) // 1 damage (one state)
        }
        AttackType::Heavy => {
            // Heavy attack: larger, longer range hitbox
            let rect = Rect::from_center_size(
                Vec2::new(60.0, 0.0),  // 60 pixels in front
                Vec2::new(60.0, 80.0), // Larger hitbox
            );
            Hitbox::new(rect, 2).with_properties(vec![AttackProperty::LightArmor])
        }
        AttackType::Grab => {
            // Grab: short range, unblockable
            let rect = Rect::from_center_size(
                Vec2::new(40.0, 0.0),  // 40 pixels in front (close range)
                Vec2::new(50.0, 70.0),
            );
            Hitbox::new(rect, 0).with_properties(vec![AttackProperty::Unblockable])
        }
    }
}

/// Debug system to log attack state changes
pub fn debug_attack_state(
    query: Query<(&Player, &CharacterState, Option<&StateTimer>), Changed<CharacterState>>,
) {
    for (player, state, timer) in query.iter() {
        if let CharacterState::Attacking { attack_type, phase } = state {
            let frames = timer.map(|t| t.target).unwrap_or(0);
            debug!(
                "Player {:?} | {:?} attack | Phase: {:?} ({} frames)",
                player, attack_type, phase, frames
            );
        }
    }
}

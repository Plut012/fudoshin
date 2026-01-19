use bevy::prelude::*;
use crate::components::character::Player;
use crate::components::combat::{AttackProperty, Hitbox};
use crate::components::movelist::{AttackDirection, Movelist};
use crate::components::state::*;
use crate::systems::input::CurrentInputs;

/// Progress attack animations through phases (Startup → Active → Recovery → Idle)
pub fn progress_attack_phases(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CharacterState, &mut StateTimer)>,
) {
    let count = query.iter().count();
    if count > 0 {
        debug!("progress_attack_phases: Found {} entities with StateTimer", count);
    }

    for (entity, mut state, mut timer) in query.iter_mut() {
        debug!("Checking entity {:?}, state: {:?}", entity, state);
        if let CharacterState::Attacking { attack_type, direction, phase, .. } = *state {
            let before_tick = timer.elapsed;
            timer.tick();
            debug!("Timer TICK: {}/{} -> {}/{} for {:?} {:?} {:?}",
                before_tick, timer.target, timer.elapsed, timer.target,
                attack_type, direction, phase);

            if timer.is_complete() {
                // Transition to next phase
                match phase {
                    AttackPhase::Startup => {
                        // Startup complete → Enter Active phase
                        let attack_data = get_attack_data(attack_type);
                        *state = CharacterState::Attacking {
                            attack_type,
                            direction,
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
                            direction,
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

                        info!("Attack phase: Recovery → Idle, REMOVING StateTimer from entity {:?}", entity);
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
    mut query: Query<(Entity, &Player, &Transform, &mut CharacterState, &Movelist)>,
    opponent_query: Query<&Transform, (With<Player>, Without<CharacterState>)>,
) {
    // Get both player positions for direction calculation
    let positions: Vec<(Entity, Vec2)> = query.iter().map(|(e, _, t, _, _)| (e, t.translation.xy())).collect();

    for (entity, player, transform, mut state, movelist) in query.iter_mut() {
        // Can only attack from Idle or Walking state
        if !matches!(*state, CharacterState::Idle | CharacterState::Walking) {
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
            // Get opponent position for direction calculation
            let player_x = transform.translation.x;
            let opponent_x = positions.iter()
                .find(|(e, _)| *e != entity)
                .map(|(_, pos)| pos.x)
                .unwrap_or(0.0);

            // Get attack direction from input
            let direction = input.get_attack_direction(*player, opponent_x, player_x);

            // Get move data from movelist
            let move_data = match movelist.get_move(attack_type, direction) {
                Some(data) => data,
                None => {
                    warn!("No move found for {:?} {:?}, using neutral", attack_type, direction);
                    movelist.get_move(attack_type, AttackDirection::Neutral)
                        .expect("Neutral move must exist!")
                }
            };

            // Enter Attacking state
            *state = CharacterState::Attacking {
                attack_type,
                direction,
                phase: AttackPhase::Startup,
            };

            // Cancel dash/evade if one is active (dash and evade are cancellable into attacks)
            commands.entity(entity).remove::<crate::systems::movement::DashData>();
            commands.entity(entity).remove::<crate::systems::evade::EvadeData>();

            // Add timer for startup phase
            let timer = StateTimer::new(move_data.startup_frames);
            info!("Adding StateTimer: target={}, move={:?}", timer.target, direction);
            commands.entity(entity).insert(timer);

            // Add/update hitbox component with move data
            let hitbox = Hitbox {
                rect: Rect::from_center_size(move_data.hitbox_offset, move_data.hitbox_size),
                active: false,
                damage: move_data.damage as u8,
                properties: move_data.properties.clone(),
            };
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.insert(hitbox);
            }

            info!(
                "Attack started: {:?} {:?} ({}f startup) for entity {:?}",
                attack_type, direction, move_data.startup_frames, entity
            );

            debug!(
                "Player {:?} initiated {:?} attack (startup: {}f)",
                player, attack_type, move_data.startup_frames
            );
        }
    }
}

/// Create a hitbox for the given attack type
pub fn create_hitbox(attack_type: AttackType) -> Hitbox {
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
        if let CharacterState::Attacking { attack_type, phase, .. } = state {
            let frames = timer.map(|t| t.target).unwrap_or(0);
            debug!(
                "Player {:?} | {:?} attack | Phase: {:?} ({} frames)",
                player, attack_type, phase, frames
            );
        }
    }
}

/// Visual indicator showing attack direction
pub fn visualize_attack_direction(
    mut gizmos: Gizmos,
    query: Query<(&CharacterState, &Transform, &Player)>,
) {
    use crate::components::movelist::AttackDirection;

    for (state, transform, _player) in query.iter() {
        if let CharacterState::Attacking { direction, phase, .. } = state {
            // Only show during Startup and Active phases (not Recovery)
            if !matches!(phase, AttackPhase::Startup | AttackPhase::Active) {
                continue;
            }

            let pos = transform.translation.truncate();
            let indicator_distance = 50.0;

            // Draw direction indicator based on attack direction
            let (indicator_pos, indicator_color) = match direction {
                AttackDirection::Neutral => {
                    // Circle for neutral
                    gizmos.circle_2d(
                        pos + Vec2::new(0.0, 80.0),
                        8.0,
                        Color::srgb(1.0, 1.0, 0.0), // Yellow
                    );
                    continue;
                }
                AttackDirection::Forward => {
                    (pos + Vec2::new(indicator_distance, 60.0), Color::srgb(0.0, 1.0, 0.0)) // Green
                }
                AttackDirection::Down => {
                    (pos + Vec2::new(0.0, -60.0), Color::srgb(1.0, 0.5, 0.0)) // Orange
                }
                AttackDirection::Back => {
                    (pos + Vec2::new(-indicator_distance, 60.0), Color::srgb(0.5, 0.5, 1.0)) // Light blue
                }
            };

            // Draw arrow or indicator
            match direction {
                AttackDirection::Forward => {
                    // Right arrow
                    let arrow_size = 15.0;
                    gizmos.line_2d(
                        indicator_pos - Vec2::new(arrow_size, 0.0),
                        indicator_pos + Vec2::new(arrow_size, 0.0),
                        indicator_color,
                    );
                    gizmos.line_2d(
                        indicator_pos + Vec2::new(arrow_size, 0.0),
                        indicator_pos + Vec2::new(arrow_size - 8.0, 8.0),
                        indicator_color,
                    );
                    gizmos.line_2d(
                        indicator_pos + Vec2::new(arrow_size, 0.0),
                        indicator_pos + Vec2::new(arrow_size - 8.0, -8.0),
                        indicator_color,
                    );
                }
                AttackDirection::Down => {
                    // Down arrow
                    let arrow_size = 15.0;
                    gizmos.line_2d(
                        indicator_pos - Vec2::new(0.0, arrow_size),
                        indicator_pos + Vec2::new(0.0, arrow_size),
                        indicator_color,
                    );
                    gizmos.line_2d(
                        indicator_pos - Vec2::new(0.0, arrow_size),
                        indicator_pos + Vec2::new(8.0, -arrow_size + 8.0),
                        indicator_color,
                    );
                    gizmos.line_2d(
                        indicator_pos - Vec2::new(0.0, arrow_size),
                        indicator_pos + Vec2::new(-8.0, -arrow_size + 8.0),
                        indicator_color,
                    );
                }
                AttackDirection::Back => {
                    // Left arrow
                    let arrow_size = 15.0;
                    gizmos.line_2d(
                        indicator_pos - Vec2::new(arrow_size, 0.0),
                        indicator_pos + Vec2::new(arrow_size, 0.0),
                        indicator_color,
                    );
                    gizmos.line_2d(
                        indicator_pos - Vec2::new(arrow_size, 0.0),
                        indicator_pos + Vec2::new(-arrow_size + 8.0, 8.0),
                        indicator_color,
                    );
                    gizmos.line_2d(
                        indicator_pos - Vec2::new(arrow_size, 0.0),
                        indicator_pos + Vec2::new(-arrow_size + 8.0, -8.0),
                        indicator_color,
                    );
                }
                AttackDirection::Neutral => {} // Already handled with circle above
            }
        }
    }
}

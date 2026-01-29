use bevy::prelude::*;
use crate::components::character::Player;
use crate::components::initiative::Initiative;
use crate::components::movelist::{AttackDirection, Movelist};
use crate::components::state::{AttackPhase, CharacterState};
use crate::components::stumble::{StumbleDirection, StumbleState};
use crate::components::combat::StumbleProperty;
use crate::events::combat_events::HitEvent;
use crate::systems::input::CurrentInputs;

// ==================== APPLY STUMBLE ON HIT ====================

/// Apply stumble state when launcher moves hit
pub fn apply_stumble_on_hit(
    mut commands: Commands,
    mut hit_events: EventReader<HitEvent>,
    attacker_query: Query<(&CharacterState, &Movelist)>,
    defender_query: Query<&StumbleState>,
) {
    for event in hit_events.read() {
        // Only unblocked hits cause stumble
        if event.was_blocked {
            continue;
        }

        // Don't apply launcher if defender is already stumbling
        // (Extensions and spikes handle stumbling opponents)
        if defender_query.get(event.defender).is_ok() {
            continue;
        }

        // Get attacker's current move
        if let Ok((state, movelist)) = attacker_query.get(event.attacker) {
            if let CharacterState::Attacking { attack_type, direction, phase } = state {
                // Only apply stumble during active phase
                if *phase != AttackPhase::Active {
                    continue;
                }

                // Get move data
                if let Some(move_data) = movelist.get_move(*attack_type, *direction) {
                    match &move_data.stumble_property {
                        StumbleProperty::Launcher(stumble_dir, duration) => {
                            let can_tech = !event.counter_hit;  // Counter hits can't be teched
                            let mut stumble = StumbleState::new(
                                *stumble_dir,
                                *duration,
                                can_tech,
                            );

                            if event.counter_hit {
                                stumble.from_counter_hit = true;
                                stumble.frames_remaining += 10;  // Bonus duration on counter hit
                            }

                            commands.entity(event.defender).insert(stumble);

                            info!(
                                "LAUNCHER: {:?} {:?} → Stumble {:?} direction, {}f, can_tech: {}",
                                attack_type, direction, stumble_dir, duration, can_tech
                            );
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

// ==================== EXTEND STUMBLE ====================

/// Extend existing stumble when extender moves hit during stumble
pub fn extend_stumble_on_hit(
    mut hit_events: EventReader<HitEvent>,
    attacker_query: Query<(&CharacterState, &Movelist)>,
    mut defender_query: Query<&mut StumbleState>,
) {
    for event in hit_events.read() {
        if event.was_blocked {
            continue;
        }

        // Check if defender is already stumbling
        if let Ok(mut stumble) = defender_query.get_mut(event.defender) {
            // Check if attacker used an extender move
            if let Ok((state, movelist)) = attacker_query.get(event.attacker) {
                if let CharacterState::Attacking { attack_type, direction, phase } = state {
                    if *phase != AttackPhase::Active {
                        continue;
                    }

                    if let Some(move_data) = movelist.get_move(*attack_type, *direction) {
                        if let StumbleProperty::Extender(extend_dir, extend_frames) = &move_data.stumble_property {
                            // Extend the stumble!
                            let before_count = stumble.extension_count;
                            stumble.extend(*extend_dir, *extend_frames);

                            if stumble.extension_count > before_count {
                                info!(
                                    "EXTENDER: {:?} {:?} → Stumble extended to {:?}, extension #{}",
                                    attack_type, direction, extend_dir, stumble.extension_count
                                );
                            } else {
                                info!("EXTENDER: Max extensions reached, stumble not extended");
                            }
                        }
                    }
                }
            }
        }
    }
}

// ==================== TECH SYSTEM ====================

/// Handle tech input during stumble state
///
/// Any attack button during the 8-frame tech window removes stumble
/// and applies -5 frame disadvantage to the defender
pub fn handle_tech_input(
    mut commands: Commands,
    inputs: Res<CurrentInputs>,
    mut query: Query<(Entity, &Player, &mut StumbleState)>,
) {
    for (entity, player, mut stumble) in query.iter_mut() {
        // Skip if can't tech
        if !stumble.can_tech {
            continue;
        }

        // Skip if not in tech window
        if !stumble.is_in_tech_window() {
            continue;
        }

        let input = match player {
            Player::One => &inputs.player_one,
            Player::Two => &inputs.player_two,
        };

        // Any attack button techs
        let tech_pressed = input.light_attack || input.heavy_attack || input.grab;

        if tech_pressed {
            // Successful tech!
            info!(
                "Player {:?} TECHED! Frame {} of tech window ({}f ago)",
                player,
                stumble.elapsed_frames,
                stumble.elapsed_frames.saturating_sub(stumble.tech_window_start)
            );

            // Remove stumble state
            commands.entity(entity).remove::<StumbleState>();

            // Apply -5 frame disadvantage (attacker still has advantage)
            commands.entity(entity).insert(Initiative { frames: -5 });

            // Add marker component for tech flash
            commands.entity(entity).insert(TechFlash { frames_remaining: 6 });
        }
    }
}

/// Marker component for tech flash visual effect
#[derive(Component)]
pub struct TechFlash {
    pub frames_remaining: u8,
}

// ==================== WALL BOUNCE ====================

/// Stage boundaries
const STAGE_HALF_WIDTH: f32 = 500.0;
const WALL_BOUNCE_THRESHOLD: f32 = STAGE_HALF_WIDTH - 30.0; // Account for character width

/// Detect wall bounce during stumble
///
/// When a stumbling player hits the stage boundary:
/// - Direction reverses
/// - +20 frames added to stumble
/// - Cannot tech during bounce
pub fn detect_wall_bounce(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut StumbleState)>,
) {
    for (entity, transform, mut stumble) in query.iter_mut() {
        let x = transform.translation.x;

        // Check if hit left or right wall
        let hit_left_wall = x <= -WALL_BOUNCE_THRESHOLD
            && matches!(stumble.direction, StumbleDirection::Backward);
        let hit_right_wall = x >= WALL_BOUNCE_THRESHOLD
            && matches!(stumble.direction, StumbleDirection::Forward);

        if hit_left_wall || hit_right_wall {
            // Apply wall bounce effect
            stumble.apply_wall_bounce();

            // Add wall bounce visual marker
            commands.entity(entity).insert(WallBounceFlash { frames_remaining: 8 });

            info!(
                "WALL BOUNCE at x={:.1}! Direction: {:?}, +20f vulnerability",
                x, stumble.direction
            );
        }
    }
}

/// Marker component for wall bounce visual effect
#[derive(Component)]
pub struct WallBounceFlash {
    pub frames_remaining: u8,
}

// ==================== SPIKE FINISHER ====================

/// Handle spike finisher moves hitting stumbling opponents
///
/// Spikes are high-damage finishing moves that:
/// - Certain heavy attacks (Neutral Heavy, Down Heavy) act as spikes when hitting stumbling opponents
/// - Deal full damage (15-16 damage)
/// - Cause hard knockdown (removes stumble)
/// - Trigger special visual effects
///
/// Note: These same moves act as Launchers when hitting non-stumbling opponents
pub fn handle_spike_finisher(
    mut commands: Commands,
    mut hit_events: EventReader<HitEvent>,
    attacker_query: Query<(&CharacterState, &Movelist)>,
    defender_query: Query<&StumbleState>,
) {
    use crate::components::state::AttackType;

    for event in hit_events.read() {
        if event.was_blocked {
            continue;
        }

        // Check if defender is stumbling
        if let Ok(_stumble) = defender_query.get(event.defender) {
            // Check if attacker used a move that can spike
            if let Ok((state, movelist)) = attacker_query.get(event.attacker) {
                if let CharacterState::Attacking { attack_type, direction, phase } = state {
                    if *phase != AttackPhase::Active {
                        continue;
                    }

                    // Moves that can spike when hitting stumbling opponents:
                    // - Neutral Heavy (armored power hit)
                    // - Down Heavy (sweep)
                    // - Explicit Spike property moves
                    let can_spike = match (attack_type, direction) {
                        (AttackType::Heavy, AttackDirection::Neutral) => true,
                        (AttackType::Heavy, AttackDirection::Down) => true,
                        _ => {
                            // Check for explicit Spike property
                            if let Some(move_data) = movelist.get_move(*attack_type, *direction) {
                                matches!(move_data.stumble_property, StumbleProperty::Spike)
                            } else {
                                false
                            }
                        }
                    };

                    if can_spike {
                        // SPIKE HIT!
                        info!(
                            "SPIKE FINISHER! {:?} {:?} landed on stumbling opponent → HARD KNOCKDOWN",
                            attack_type, direction
                        );

                        // Remove stumble state (hard knockdown)
                        commands.entity(event.defender).remove::<StumbleState>();

                        // Add spike visual marker (extra hitstop + flash)
                        commands.entity(event.defender).insert(SpikeFlash { frames_remaining: 12 });

                        // Damage is already applied by the damage system
                        // These moves have 15-16 damage in their MoveData
                    }
                }
            }
        }
    }
}

/// Marker component for spike finisher visual effect
#[derive(Component)]
pub struct SpikeFlash {
    pub frames_remaining: u8,
}

// ==================== PROCESS STUMBLE ====================

/// Tick down stumble duration each frame
pub fn process_stumble(
    mut commands: Commands,
    mut query: Query<(Entity, &mut StumbleState)>,
) {
    for (entity, mut stumble) in query.iter_mut() {
        stumble.tick();

        if stumble.should_end() {
            commands.entity(entity).remove::<StumbleState>();

            if stumble.extension_count >= 4 {
                info!("Stumble ended: max extensions (4) reached");
            } else {
                debug!("Stumble ended: duration expired");
            }
        }
    }
}

// ==================== VISUAL FEEDBACK ====================

/// Show stumble direction arrow at player's feet
pub fn visualize_stumble_direction(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &StumbleState)>,
) {
    for (transform, stumble) in query.iter() {
        let pos = transform.translation.truncate();
        let feet_pos = pos + Vec2::new(0.0, -100.0);  // Below character

        // Arrow color based on tech window
        let color = if stumble.can_tech && stumble.is_in_tech_window() {
            Color::srgb(1.0, 1.0, 0.0)  // Yellow - in tech window
        } else if stumble.can_tech {
            Color::srgb(1.0, 0.8, 0.3)  // Orange - can tech but not in window yet
        } else {
            Color::srgb(1.0, 0.3, 0.3)  // Red - cannot tech
        };

        // Draw directional arrow
        let arrow_dir = match stumble.direction {
            StumbleDirection::Backward => Vec2::new(-20.0, 0.0),
            StumbleDirection::Forward => Vec2::new(20.0, 0.0),
            StumbleDirection::Down => Vec2::new(0.0, -20.0),
        };

        // Arrow shaft
        gizmos.line_2d(feet_pos, feet_pos + arrow_dir, color);

        // Arrowhead
        let arrow_tip = feet_pos + arrow_dir;
        let perpendicular = Vec2::new(-arrow_dir.y, arrow_dir.x).normalize() * 5.0;
        gizmos.line_2d(arrow_tip, arrow_tip - arrow_dir.normalize() * 8.0 + perpendicular, color);
        gizmos.line_2d(arrow_tip, arrow_tip - arrow_dir.normalize() * 8.0 - perpendicular, color);
    }
}

/// Subtle visual feedback for stumbling state
pub fn visualize_stumble_state(
    mut query: Query<(&StumbleState, &mut Sprite, &Player)>,
) {
    for (stumble, mut sprite, player) in query.iter_mut() {
        // Base colors
        let base_color = match player {
            Player::One => Color::srgb(0.9, 0.2, 0.2),   // Red
            Player::Two => Color::srgb(0.2, 0.4, 0.9),   // Blue
        };

        // Subtle darkening to show disadvantage (90% brightness)
        let darken = 0.9;
        sprite.color = Color::srgb(
            base_color.to_srgba().red * darken,
            base_color.to_srgba().green * darken,
            base_color.to_srgba().blue * darken,
        );

        // If in tech window, add slight pulsing brightness
        if stumble.is_in_tech_window() {
            let pulse = 0.05 * ((stumble.elapsed_frames % 4) as f32 / 4.0);
            sprite.color = Color::srgb(
                (base_color.to_srgba().red * darken) + pulse,
                (base_color.to_srgba().green * darken) + pulse,
                (base_color.to_srgba().blue * darken) + pulse,
            );
        }
    }
}

/// Dark red flash on successful tech
pub fn tech_flash_effect(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TechFlash, &mut Sprite)>,
) {
    for (entity, mut flash, mut sprite) in query.iter_mut() {
        if flash.frames_remaining > 0 {
            // Dark red flash (subtle)
            let intensity = flash.frames_remaining as f32 / 6.0;
            sprite.color = Color::srgb(0.6 * intensity, 0.1 * intensity, 0.1 * intensity);

            flash.frames_remaining -= 1;
        } else {
            // Flash complete, remove component
            commands.entity(entity).remove::<TechFlash>();
        }
    }
}

/// Visual effect for wall bounce (expanding impact circles)
pub fn wall_bounce_visual(
    mut commands: Commands,
    mut gizmos: Gizmos,
    mut query: Query<(Entity, &Transform, &mut WallBounceFlash)>,
) {
    for (entity, transform, mut flash) in query.iter_mut() {
        if flash.frames_remaining > 0 {
            let pos = transform.translation.truncate();

            // Multiple expanding circles for impact effect
            let progress = 1.0 - (flash.frames_remaining as f32 / 8.0);
            let base_radius = 40.0;

            for i in 0..3 {
                let radius = base_radius + (progress * 60.0) + (i as f32 * 20.0);
                let alpha = (1.0 - progress) * 0.8;
                let color = Color::srgba(1.0, 0.5, 0.0, alpha); // Orange impact

                gizmos.circle_2d(pos, radius, color);
            }

            flash.frames_remaining -= 1;
        } else {
            // Effect complete
            commands.entity(entity).remove::<WallBounceFlash>();
        }
    }
}

/// Intense visual effect for spike finisher
///
/// Creates a powerful impact flash with expanding shockwave
pub fn spike_finisher_visual(
    mut commands: Commands,
    mut gizmos: Gizmos,
    mut query: Query<(Entity, &Transform, &mut SpikeFlash, &mut Sprite)>,
) {
    for (entity, transform, mut flash, mut sprite) in query.iter_mut() {
        if flash.frames_remaining > 0 {
            let pos = transform.translation.truncate();
            let progress = 1.0 - (flash.frames_remaining as f32 / 12.0);

            // Intense white flash for the first few frames
            if flash.frames_remaining > 8 {
                sprite.color = Color::srgb(1.0, 1.0, 1.0);
            } else {
                // Fade to red/orange impact color
                let intensity = flash.frames_remaining as f32 / 8.0;
                sprite.color = Color::srgb(1.0, 0.3 * intensity, 0.0);
            }

            // Multiple expanding shockwave circles
            let base_radius = 50.0;
            for i in 0..5 {
                let radius = base_radius + (progress * 120.0) + (i as f32 * 25.0);
                let alpha = (1.0 - progress) * 0.9;

                // Alternate red and white circles for dramatic effect
                let color = if i % 2 == 0 {
                    Color::srgba(1.0, 0.0, 0.0, alpha)  // Red
                } else {
                    Color::srgba(1.0, 1.0, 1.0, alpha)  // White
                };

                gizmos.circle_2d(pos, radius, color);
            }

            // Cross-shaped impact lines
            let line_length = 80.0 * (1.0 - progress);
            let alpha = 1.0 - progress;
            let line_color = Color::srgba(1.0, 1.0, 0.0, alpha); // Yellow impact

            gizmos.line_2d(
                pos + Vec2::new(-line_length, 0.0),
                pos + Vec2::new(line_length, 0.0),
                line_color,
            );
            gizmos.line_2d(
                pos + Vec2::new(0.0, -line_length),
                pos + Vec2::new(0.0, line_length),
                line_color,
            );

            flash.frames_remaining -= 1;
        } else {
            // Effect complete
            commands.entity(entity).remove::<SpikeFlash>();
        }
    }
}

/// Debug: Log stumble state changes
pub fn debug_stumble_state(
    query: Query<(&Player, &StumbleState), Changed<StumbleState>>,
) {
    for (player, stumble) in query.iter() {
        debug!(
            "Player {:?} stumbling: {:?} direction, {}f remaining, extension #{}, can_tech: {}, in_window: {}",
            player,
            stumble.direction,
            stumble.frames_remaining,
            stumble.extension_count,
            stumble.can_tech,
            stumble.is_in_tech_window()
        );
    }
}

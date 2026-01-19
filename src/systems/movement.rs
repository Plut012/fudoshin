use bevy::prelude::*;
use crate::components::character::*;
use crate::components::state::*;
use crate::systems::input::CurrentInputs;

use crate::components::state::StateTimer;

const STAGE_WIDTH: f32 = 1000.0;
const STAGE_HALF_WIDTH: f32 = STAGE_WIDTH / 2.0;

use crate::systems::evade::EvadeData;

/// Process player inputs and update velocities
pub fn process_movement_input(
    inputs: Res<CurrentInputs>,
    mut query: Query<(&Player, &mut Velocity, &MaxSpeed, &CharacterState, Option<&EvadeData>, Option<&DashData>)>,
) {
    for (player, mut velocity, max_speed, state, evade_data, dash_data) in query.iter_mut() {
        // Don't override evade or dash movement
        if evade_data.is_some() || dash_data.is_some() {
            continue;
        }

        // Only allow movement in Idle or Walking states
        // Can't move while attacking, blocking, parrying, or staggered
        if !matches!(state, CharacterState::Idle | CharacterState::Walking) {
            velocity.0.x = 0.0; // Stop moving
            continue;
        }

        // Get input for this player
        let input = match player {
            Player::One => &inputs.player_one,
            Player::Two => &inputs.player_two,
        };

        // Update horizontal velocity based on input
        if input.movement.x != 0.0 {
            velocity.0.x = input.movement.x * max_speed.0;
        } else {
            // Stop immediately when no input (responsive feel)
            velocity.0.x = 0.0;
        }
    }
}

/// Update character states based on velocity
pub fn update_movement_state(
    mut query: Query<(&Velocity, &mut CharacterState), Changed<Velocity>>,
) {
    for (velocity, mut state) in query.iter_mut() {
        // Only update if in movement states
        if !matches!(*state, CharacterState::Idle | CharacterState::Walking) {
            continue;
        }

        // Don't clean up StateTimer here - let the systems that add it manage removal
        // (attack, guard, etc. systems handle their own StateTimer lifecycle)

        if velocity.0.x.abs() > 0.1 {
            *state = CharacterState::Walking;
        } else {
            *state = CharacterState::Idle;
        }
    }
}

/// Apply velocity to transform positions
pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform), With<Character>>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
    }
}

/// Clamp characters to stage boundaries
pub fn clamp_to_stage(
    mut query: Query<&mut Transform, With<Character>>,
) {
    for mut transform in query.iter_mut() {
        // Clamp X position to stage boundaries
        transform.translation.x = transform.translation.x.clamp(
            -STAGE_HALF_WIDTH + 30.0,  // Half character width
            STAGE_HALF_WIDTH - 30.0,
        );

        // Keep characters on the ground (Y = 0 for now)
        transform.translation.y = 0.0;
    }
}

/// Debug system to visualize character state
pub fn debug_character_state(
    query: Query<(&Player, &CharacterState, &Transform), Changed<CharacterState>>,
) {
    for (player, state, transform) in query.iter() {
        debug!(
            "Player {:?} | State: {:?} | Pos: {:.1}",
            player,
            state,
            transform.translation.x
        );
    }
}

// ============================================================================
// ATTACK MOVEMENT SYSTEMS
// ============================================================================

use crate::components::movelist::{AttackDirection, AttackMovement, Movelist};

/// Component to track attack movement progress
#[derive(Component, Debug)]
pub struct ActiveAttackMovement {
    /// Total distance to move
    pub total_distance: f32,
    /// Distance moved so far
    pub distance_moved: f32,
    /// Speed per frame
    pub speed: f32,
    /// Direction modifier (1.0 for facing right, -1.0 for facing left)
    pub facing_multiplier: f32,
}

impl ActiveAttackMovement {
    pub fn new(movement: &AttackMovement, facing_right: bool) -> Self {
        let facing_multiplier = if facing_right { 1.0 } else { -1.0 };
        Self {
            total_distance: movement.distance,
            distance_moved: 0.0,
            speed: movement.speed,
            facing_multiplier,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.distance_moved.abs() >= self.total_distance.abs()
    }

    pub fn remaining_distance(&self) -> f32 {
        self.total_distance - self.distance_moved
    }
}

/// Initiate attack movement when entering Active phase with a move that has movement
pub fn initiate_attack_movement(
    mut commands: Commands,
    query: Query<(Entity, &CharacterState, &Movelist, &Transform), Changed<CharacterState>>,
    opponent_query: Query<&Transform, With<Player>>,
) {
    for (entity, state, movelist, transform) in query.iter() {
        // Only start movement when entering Active phase
        if let CharacterState::Attacking {
            attack_type,
            direction,
            phase: AttackPhase::Active,
        } = state
        {
            // Get the move data
            if let Some(move_data) = movelist.get_move(*attack_type, *direction) {
                // Check if this move has movement
                if let Some(ref movement) = move_data.movement {
                    // Determine facing direction (based on opponent position)
                    let player_x = transform.translation.x;
                    let opponent_x = opponent_query
                        .iter()
                        .find(|t| t.translation.x != player_x)
                        .map(|t| t.translation.x)
                        .unwrap_or(0.0);

                    let facing_right = opponent_x > player_x;

                    // Add movement tracking component
                    let active_movement = ActiveAttackMovement::new(movement, facing_right);
                    commands.entity(entity).insert(active_movement);

                    debug!(
                        "Attack movement initiated: distance={}, speed={}",
                        movement.distance, movement.speed
                    );
                }
            }
        }
    }
}

/// Apply attack movement gradually during Active phase
pub fn apply_attack_movement(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &CharacterState,
        &mut Transform,
        &mut ActiveAttackMovement,
    )>,
) {
    for (entity, state, mut transform, mut movement) in query.iter_mut() {
        // Only apply movement during Active phase
        if let CharacterState::Attacking {
            phase: AttackPhase::Active,
            ..
        } = state
        {
            // Calculate how much to move this frame
            let remaining = movement.remaining_distance();
            let move_amount = if remaining.abs() < movement.speed {
                remaining
            } else {
                remaining.signum() * movement.speed
            };

            // Apply movement (with facing direction)
            transform.translation.x += move_amount * movement.facing_multiplier;
            movement.distance_moved += move_amount;

            // Check if movement is complete
            if movement.is_complete() {
                debug!(
                    "Attack movement complete: total moved = {}",
                    movement.distance_moved
                );
            }
        } else {
            // Not in Active phase anymore - remove component
            commands.entity(entity).remove::<ActiveAttackMovement>();
        }
    }
}

/// Clean up movement component when attack ends
pub fn cleanup_attack_movement(
    mut commands: Commands,
    query: Query<(Entity, &CharacterState, Option<&ActiveAttackMovement>)>,
) {
    for (entity, state, movement) in query.iter() {
        // If we have a movement component but are no longer attacking, remove it
        if movement.is_some() && !matches!(state, CharacterState::Attacking { .. }) {
            commands.entity(entity).remove::<ActiveAttackMovement>();
            debug!("Attack movement cleaned up");
        }
    }
}

// ============================================================================
// DASH SYSTEM
// ============================================================================

const DASH_DISTANCE: f32 = 120.0;  // Fixed distance for dash
const DASH_SPEED: f32 = 20.0;      // Speed per frame
const DASH_COOLDOWN: u32 = 30;     // 30 frames = 0.5 seconds at 60fps

/// Component to track dash state and cooldown
#[derive(Component, Debug)]
pub struct DashData {
    /// Direction of the dash (-1.0 or 1.0)
    pub direction: f32,
    /// Distance traveled so far
    pub distance_traveled: f32,
}

/// Component to track dash cooldown
#[derive(Component, Debug)]
pub struct DashCooldown {
    pub frames_remaining: u32,
}

impl DashCooldown {
    pub fn new() -> Self {
        Self {
            frames_remaining: DASH_COOLDOWN,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.frames_remaining == 0
    }

    pub fn tick(&mut self) {
        if self.frames_remaining > 0 {
            self.frames_remaining -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.frames_remaining = DASH_COOLDOWN;
    }
}

/// Handle dash input (Shift + direction)
pub fn handle_dash_input(
    mut commands: Commands,
    inputs: Res<CurrentInputs>,
    mut query: Query<(Entity, &Player, &CharacterState, Option<&mut DashCooldown>, Option<&DashData>)>,
) {
    for (entity, player, state, cooldown, dash_data) in query.iter_mut() {
        // Can only dash from Idle or Walking state
        if !matches!(state, CharacterState::Idle | CharacterState::Walking) {
            continue;
        }

        // Already dashing
        if dash_data.is_some() {
            continue;
        }

        // Check cooldown
        let can_dash = cooldown.as_ref().map(|cd| cd.is_ready()).unwrap_or(true);
        if !can_dash {
            continue;
        }

        // Get input for this player
        let input = match player {
            Player::One => &inputs.player_one,
            Player::Two => &inputs.player_two,
        };

        // Check for dash input (step flag is set when shift + direction is pressed)
        if input.step && input.movement.x.abs() > 0.1 {
            // Initiate dash
            let direction = input.movement.x.signum();
            commands.entity(entity).insert(DashData {
                direction,
                distance_traveled: 0.0,
            });

            // Reset/add cooldown
            if let Some(mut cd) = cooldown {
                cd.reset();
            } else {
                commands.entity(entity).insert(DashCooldown::new());
            }

            info!("Player {:?} dashing {:}", player, if direction > 0.0 { "forward" } else { "backward" });
        }
    }
}

/// Apply dash movement
pub fn apply_dash_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut DashData, &CharacterState)>,
) {
    for (entity, mut transform, mut dash, state) in query.iter_mut() {
        // If we're attacking, cancel the dash immediately
        if matches!(state, CharacterState::Attacking { .. }) {
            commands.entity(entity).remove::<DashData>();
            debug!("Dash cancelled by attack");
            continue;
        }

        // Calculate how much to move this frame
        let remaining = DASH_DISTANCE - dash.distance_traveled;
        let move_amount = if remaining < DASH_SPEED {
            remaining
        } else {
            DASH_SPEED
        };

        // Apply movement
        transform.translation.x += move_amount * dash.direction;
        dash.distance_traveled += move_amount;

        // Check if dash is complete
        if dash.distance_traveled >= DASH_DISTANCE {
            commands.entity(entity).remove::<DashData>();
            debug!("Dash complete: {} units", dash.distance_traveled);
        }
    }
}

/// Tick down dash cooldowns
pub fn tick_dash_cooldown(
    mut query: Query<&mut DashCooldown>,
) {
    for mut cooldown in query.iter_mut() {
        cooldown.tick();
    }
}

/// Visual indicator for dash cooldown
pub fn visualize_dash_cooldown(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Player, Option<&DashCooldown>)>,
) {
    for (transform, _player, cooldown) in query.iter() {
        if let Some(cooldown) = cooldown {
            if !cooldown.is_ready() {
                // Draw cooldown indicator
                let pos = transform.translation.truncate();
                let cooldown_ratio = cooldown.frames_remaining as f32 / DASH_COOLDOWN as f32;

                // Draw a small arc showing cooldown progress
                let radius = 25.0;
                let color = Color::srgba(0.8, 0.8, 0.8, 0.5);

                gizmos.circle_2d(
                    pos + Vec2::new(0.0, -70.0),
                    radius * (1.0 - cooldown_ratio),
                    color,
                );
            }
        }
    }
}

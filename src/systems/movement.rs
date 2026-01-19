use bevy::prelude::*;
use crate::components::character::*;
use crate::components::state::*;
use crate::systems::input::CurrentInputs;

const STAGE_WIDTH: f32 = 1000.0;
const STAGE_HALF_WIDTH: f32 = STAGE_WIDTH / 2.0;

use crate::systems::evade::EvadeData;

/// Process player inputs and update velocities
pub fn process_movement_input(
    inputs: Res<CurrentInputs>,
    mut query: Query<(&Player, &mut Velocity, &MaxSpeed, &CharacterState, Option<&EvadeData>)>,
) {
    for (player, mut velocity, max_speed, state, evade_data) in query.iter_mut() {
        // Don't override evade movement
        if evade_data.is_some() {
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

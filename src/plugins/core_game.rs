use bevy::prelude::*;
use crate::systems::{input, movement};

/// Core game plugin - manages fundamental game systems
/// Phase 1: Movement and input handling
pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<input::CurrentInputs>()

            // Systems - order matters for frame-perfect timing
            .add_systems(Update, (
                // 1. Read inputs first
                input::update_inputs,

                // 2. Process inputs into game actions
                movement::process_movement_input,

                // 3. Update state machine
                movement::update_movement_state,

                // 4. Apply physics
                movement::apply_velocity,

                // 5. Enforce constraints
                movement::clamp_to_stage,

                // Debug (runs independently)
                movement::debug_character_state,
            ).chain());  // Chain ensures proper execution order
    }
}

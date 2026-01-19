use bevy::prelude::*;
use crate::components::character::Player;
use crate::components::movelist::AttackDirection;

/// Raw input state for each player
#[derive(Debug, Clone, Copy, Default)]
pub struct PlayerInput {
    pub movement: Vec2,      // -1 to 1 for left/right, Y for up/down
    pub light_attack: bool,
    pub heavy_attack: bool,
    pub grab: bool,
    pub block: bool,
    pub step: bool,          // Quick dash
    pub backdash: bool,
}

impl PlayerInput {
    /// Get attack direction from movement input
    /// Takes into account which direction the player is facing
    pub fn get_attack_direction(&self, _player: Player, opponent_x: f32, player_x: f32) -> AttackDirection {
        // Determine if player is facing right (opponent is to the right)
        let facing_right = opponent_x > player_x;

        let holding_down = self.movement.y < -0.5;
        let holding_horizontal = self.movement.x.abs() > 0.5;

        // PRIORITY 1: Down takes priority when holding down + any other direction
        // This is for down+forward or down+back inputs
        if holding_down {
            return AttackDirection::Down;
        }

        // PRIORITY 2: Horizontal input (forward/back)
        if holding_horizontal {
            // Forward = towards opponent, Back = away from opponent
            let is_forward = (facing_right && self.movement.x > 0.5)
                          || (!facing_right && self.movement.x < -0.5);

            if is_forward {
                AttackDirection::Forward
            } else {
                AttackDirection::Back
            }
        } else {
            // No direction = neutral
            AttackDirection::Neutral
        }
    }
}

/// Get Player 1 input from keyboard
pub fn get_p1_input(keys: &ButtonInput<KeyCode>) -> PlayerInput {
    let mut input = PlayerInput::default();

    // Movement (WASD)
    if keys.pressed(KeyCode::KeyA) {
        input.movement.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        input.movement.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyW) {
        input.movement.y += 1.0;  // Up
    }
    if keys.pressed(KeyCode::KeyS) {
        input.movement.y -= 1.0;  // Down (for crouching attacks)
    }

    // Actions (JKL)
    input.light_attack = keys.just_pressed(KeyCode::KeyJ);
    input.heavy_attack = keys.just_pressed(KeyCode::KeyK);
    input.grab = keys.just_pressed(KeyCode::KeyL);
    input.block = keys.pressed(KeyCode::KeyI);  // I for block

    // Movement options (Shift + direction) - only for evade, not attacks
    let shift = keys.pressed(KeyCode::ShiftLeft);
    if shift && input.movement.x != 0.0 {
        input.step = true;
    }

    input
}

/// Get Player 2 input from keyboard
pub fn get_p2_input(keys: &ButtonInput<KeyCode>) -> PlayerInput {
    let mut input = PlayerInput::default();

    // Movement (Arrow keys)
    if keys.pressed(KeyCode::ArrowLeft) {
        input.movement.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        input.movement.x += 1.0;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        input.movement.y += 1.0;  // Up
    }
    if keys.pressed(KeyCode::ArrowDown) {
        input.movement.y -= 1.0;  // Down (for crouching attacks)
    }

    // Actions (Numpad 1/2/3)
    input.light_attack = keys.just_pressed(KeyCode::Numpad1);
    input.heavy_attack = keys.just_pressed(KeyCode::Numpad2);
    input.grab = keys.just_pressed(KeyCode::Numpad3);
    input.block = keys.pressed(KeyCode::Numpad0);

    // Movement options (RShift + direction) - only for evade, not attacks
    let shift = keys.pressed(KeyCode::ShiftRight);
    if shift && input.movement.x != 0.0 {
        input.step = true;
    }

    input
}

/// Resource to store current frame's inputs
#[derive(Resource, Default)]
pub struct CurrentInputs {
    pub player_one: PlayerInput,
    pub player_two: PlayerInput,
}

/// System to update input resource each frame
pub fn update_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut inputs: ResMut<CurrentInputs>,
) {
    inputs.player_one = get_p1_input(&keys);
    inputs.player_two = get_p2_input(&keys);
}

use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::state::AttackType;
use crate::components::combat::AttackProperty;

/// Direction of directional attack input
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttackDirection {
    /// No direction held (standing attack)
    Neutral,
    /// Forward (towards opponent)
    Forward,
    /// Down (crouching)
    Down,
    /// Back (away from opponent)
    Back,
}

impl Default for AttackDirection {
    fn default() -> Self {
        AttackDirection::Neutral
    }
}

/// Unique identifier for a move
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MoveId {
    pub attack_type: AttackType,
    pub direction: AttackDirection,
}

impl MoveId {
    pub fn new(attack_type: AttackType, direction: AttackDirection) -> Self {
        Self { attack_type, direction }
    }
}

/// Movement properties for attacks that move the character
#[derive(Debug, Clone, Copy)]
pub struct AttackMovement {
    /// Distance to move (positive = forward, negative = backward)
    pub distance: f32,
    /// Speed of movement (units per frame)
    pub speed: f32,
}

impl AttackMovement {
    pub fn new(distance: f32, speed: f32) -> Self {
        Self { distance, speed }
    }

    /// Forward advancing movement
    pub fn forward(distance: f32) -> Self {
        Self::new(distance, 15.0)
    }

    /// Backward retreating movement
    pub fn back(distance: f32) -> Self {
        Self::new(-distance, 15.0)
    }
}

/// Complete definition of a single move
#[derive(Debug, Clone)]
pub struct MoveData {
    /// Display name of the move
    pub name: String,

    // Frame data
    /// Frames before hitbox becomes active
    pub startup_frames: u32,
    /// Frames hitbox remains active
    pub active_frames: u32,
    /// Frames of recovery after hitbox deactivates
    pub recovery_frames: u32,

    // Damage & advantage
    /// Base damage dealt
    pub damage: f32,
    /// Frame advantage on block (negative = disadvantage)
    pub on_block: i32,

    // Hitbox definition
    /// Offset from character position (local space)
    pub hitbox_offset: Vec2,
    /// Size of the hitbox
    pub hitbox_size: Vec2,

    // Properties
    /// Special properties (armor, unblockable, etc.)
    pub properties: Vec<AttackProperty>,

    // Movement
    /// Optional movement during attack
    pub movement: Option<AttackMovement>,

    // Hitstop (freeze frames on hit)
    /// Hitstop frames on normal hit
    pub hitstop_on_hit: u32,
    /// Hitstop frames when blocked
    pub hitstop_on_block: u32,
    /// Hitstop frames on counter hit
    pub hitstop_on_counter: u32,
}

impl MoveData {
    /// Get total frame duration
    pub fn total_frames(&self) -> u32 {
        self.startup_frames + self.active_frames + self.recovery_frames
    }

    /// Get hitstun frames (for damage system)
    pub fn hitstun_frames(&self) -> u32 {
        // Rough formula: more damage = more hitstun
        (self.damage * 2.0) as u32 + 10
    }

    /// Get appropriate hitstop based on hit type
    pub fn get_hitstop(&self, is_blocked: bool, is_counter: bool) -> u32 {
        if is_counter {
            self.hitstop_on_counter
        } else if is_blocked {
            self.hitstop_on_block
        } else {
            self.hitstop_on_hit
        }
    }
}

/// Component that holds a character's complete movelist
#[derive(Component, Debug, Clone)]
pub struct Movelist {
    moves: HashMap<MoveId, MoveData>,
}

impl Movelist {
    /// Create empty movelist
    pub fn new() -> Self {
        Self {
            moves: HashMap::new(),
        }
    }

    /// Add a move to the movelist
    pub fn add_move(&mut self, attack_type: AttackType, direction: AttackDirection, move_data: MoveData) {
        let move_id = MoveId::new(attack_type, direction);
        self.moves.insert(move_id, move_data);
    }

    /// Get move data for a specific attack
    pub fn get_move(&self, attack_type: AttackType, direction: AttackDirection) -> Option<&MoveData> {
        let move_id = MoveId::new(attack_type, direction);
        self.moves.get(&move_id)
    }

    /// Check if a move exists
    pub fn has_move(&self, attack_type: AttackType, direction: AttackDirection) -> bool {
        let move_id = MoveId::new(attack_type, direction);
        self.moves.contains_key(&move_id)
    }

    /// Create a default character movelist with all 8 directional attacks
    pub fn default_character() -> Self {
        let mut movelist = Self::new();

        // === LIGHT ATTACKS ===

        // Neutral Light: Standard jab
        movelist.add_move(
            AttackType::Light,
            AttackDirection::Neutral,
            MoveData {
                name: "Jab".to_string(),
                startup_frames: 5,  // Reduced from 6
                active_frames: 2,
                recovery_frames: 10,
                damage: 8.0,
                on_block: -2,
                hitbox_offset: Vec2::new(50.0, 0.0),  // Scaled by 1.25x
                hitbox_size: Vec2::new(150.0, 119.0),  // Scaled by 1.25x (1.5x character width)
                properties: vec![],
                movement: None,
                hitstop_on_hit: 9,
                hitstop_on_block: 6,
                hitstop_on_counter: 12,
            },
        );

        // Forward Light: Fast advancing poke
        movelist.add_move(
            AttackType::Light,
            AttackDirection::Forward,
            MoveData {
                name: "Dash Jab".to_string(),
                startup_frames: 3,  // Reduced from 4
                active_frames: 2,
                recovery_frames: 10,
                damage: 6.0,
                on_block: -2,
                hitbox_offset: Vec2::new(62.5, 0.0),  // Scaled by 1.25x
                hitbox_size: Vec2::new(163.0, 119.0),  // Scaled by 1.25x (1.6x character width, lunging)
                properties: vec![],
                movement: Some(AttackMovement::forward(50.0)),
                hitstop_on_hit: 8,
                hitstop_on_block: 6,
                hitstop_on_counter: 11,
            },
        );

        // Down Light: Low poke
        movelist.add_move(
            AttackType::Light,
            AttackDirection::Down,
            MoveData {
                name: "Low Poke".to_string(),
                startup_frames: 5,  // Reduced from 7
                active_frames: 2,
                recovery_frames: 11,
                damage: 7.0,
                on_block: -3,
                hitbox_offset: Vec2::new(50.0, -37.5),  // Lower hitbox, scaled by 1.25x
                hitbox_size: Vec2::new(156.0, 81.0),  // Scaled by 1.25x (1.5x character width, low)
                properties: vec![],
                movement: None,
                hitstop_on_hit: 8,
                hitstop_on_block: 6,
                hitstop_on_counter: 11,
            },
        );

        // Back Light: Safe retreating jab
        movelist.add_move(
            AttackType::Light,
            AttackDirection::Back,
            MoveData {
                name: "Step Jab".to_string(),
                startup_frames: 4,  // Reduced from 5
                active_frames: 2,
                recovery_frames: 9,
                damage: 6.0,
                on_block: 1,  // Positive on block (safe)
                hitbox_offset: Vec2::new(44.0, 0.0),  // Scaled by 1.25x
                hitbox_size: Vec2::new(144.0, 119.0),  // Scaled by 1.25x (1.4x character width, defensive)
                properties: vec![],
                movement: Some(AttackMovement::back(30.0)),
                hitstop_on_hit: 8,
                hitstop_on_block: 6,
                hitstop_on_counter: 11,
            },
        );

        // === HEAVY ATTACKS ===

        // Neutral Heavy: Standard power hit
        movelist.add_move(
            AttackType::Heavy,
            AttackDirection::Neutral,
            MoveData {
                name: "Heavy Strike".to_string(),
                startup_frames: 11,  // Reduced from 14
                active_frames: 4,
                recovery_frames: 18,
                damage: 15.0,
                on_block: -8,
                hitbox_offset: Vec2::new(62.5, 0.0),  // Scaled by 1.25x
                hitbox_size: Vec2::new(213.0, 163.0),  // Scaled by 1.25x (2.1x character width)
                properties: vec![AttackProperty::LightArmor],
                movement: None,
                hitstop_on_hit: 13,
                hitstop_on_block: 10,
                hitstop_on_counter: 16,
            },
        );

        // Forward Heavy: Advancing overhead
        movelist.add_move(
            AttackType::Heavy,
            AttackDirection::Forward,
            MoveData {
                name: "Lunging Strike".to_string(),
                startup_frames: 9,  // Reduced from 12
                active_frames: 4,
                recovery_frames: 18,
                damage: 13.0,
                on_block: -6,
                hitbox_offset: Vec2::new(75.0, 12.5),  // Slightly higher, scaled by 1.25x
                hitbox_size: Vec2::new(238.0, 163.0),  // Scaled by 1.25x (2.4x character width, lunging)
                properties: vec![],
                movement: Some(AttackMovement::forward(80.0)),
                hitstop_on_hit: 12,
                hitstop_on_block: 9,
                hitstop_on_counter: 15,
            },
        );

        // Down Heavy: Low sweep
        movelist.add_move(
            AttackType::Heavy,
            AttackDirection::Down,
            MoveData {
                name: "Sweep".to_string(),
                startup_frames: 13,  // Reduced from 16
                active_frames: 4,
                recovery_frames: 20,
                damage: 16.0,
                on_block: -10,
                hitbox_offset: Vec2::new(62.5, -43.75),  // Low hitbox, scaled by 1.25x
                hitbox_size: Vec2::new(250.0, 63.0),  // Scaled by 1.25x (2.5x character width, sweep)
                properties: vec![],
                movement: None,
                hitstop_on_hit: 14,
                hitstop_on_block: 11,
                hitstop_on_counter: 17,
            },
        );

        // Back Heavy: Defensive power hit
        movelist.add_move(
            AttackType::Heavy,
            AttackDirection::Back,
            MoveData {
                name: "Counter Strike".to_string(),
                startup_frames: 10,  // Reduced from 13
                active_frames: 4,
                recovery_frames: 16,
                damage: 14.0,
                on_block: -4,  // Safer than normal heavy
                hitbox_offset: Vec2::new(56.0, 0.0),  // Scaled by 1.25x
                hitbox_size: Vec2::new(200.0, 163.0),  // Scaled by 1.25x (2.0x character width, defensive)
                properties: vec![],
                movement: Some(AttackMovement::back(40.0)),
                hitstop_on_hit: 13,
                hitstop_on_block: 10,
                hitstop_on_counter: 16,
            },
        );

        // === GRAB (Neutral only for now) ===
        movelist.add_move(
            AttackType::Grab,
            AttackDirection::Neutral,
            MoveData {
                name: "Grab".to_string(),
                startup_frames: 10,
                active_frames: 2,
                recovery_frames: 20,
                damage: 12.0,
                on_block: 0,
                hitbox_offset: Vec2::new(44.0, 0.0),  // Scaled by 1.25x
                hitbox_size: Vec2::new(150.0, 150.0),  // Scaled by 1.25x (1.5x character width, square, very generous)
                properties: vec![AttackProperty::Unblockable],
                movement: None,
                hitstop_on_hit: 11,
                hitstop_on_block: 0,  // Can't be blocked
                hitstop_on_counter: 14,
            },
        );

        movelist
    }
}

impl Default for Movelist {
    fn default() -> Self {
        Self::default_character()
    }
}

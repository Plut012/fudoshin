use bevy::prelude::*;

/// Type of attack being performed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttackType {
    Light,
    Heavy,
    Grab,
}

/// Phase of an attack animation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackPhase {
    /// Windup before hitbox becomes active
    Startup,
    /// Hitbox is active and can hit
    Active,
    /// Cool-down after hitbox deactivates
    Recovery,
}

use crate::components::movelist::AttackDirection;

/// Character state machine
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum CharacterState {
    Idle,
    Walking,
    Stepping,
    Backdashing,
    /// Performing an attack
    Attacking {
        attack_type: AttackType,
        direction: AttackDirection,
        phase: AttackPhase,
    },
    /// Holding block
    Blocking,
    /// Attempting a parry
    Parrying { frames_remaining: u32 },
    /// Knocked back, unable to act
    Staggered { frames_remaining: u32 },
}

impl Default for CharacterState {
    fn default() -> Self {
        Self::Idle
    }
}

/// Frame timer for state transitions
#[derive(Component, Debug)]
pub struct StateTimer {
    pub elapsed: u32,
    pub target: u32,
}

impl StateTimer {
    pub fn new(target: u32) -> Self {
        Self { elapsed: 0, target }
    }

    pub fn tick(&mut self) {
        self.elapsed += 1;
    }

    pub fn is_complete(&self) -> bool {
        self.elapsed >= self.target
    }

    pub fn reset(&mut self, new_target: u32) {
        self.elapsed = 0;
        self.target = new_target;
    }
}

/// Frame data for an attack move
#[derive(Component, Debug, Clone)]
pub struct AttackData {
    pub attack_type: AttackType,
    /// Frames before hitbox becomes active
    pub startup_frames: u32,
    /// Frames hitbox remains active
    pub active_frames: u32,
    /// Frames of recovery after hitbox deactivates
    pub recovery_frames: u32,
    /// Damage dealt (in health states)
    pub damage: u8,
    /// Frame advantage on block (negative = disadvantage)
    pub on_block: i32,
}

impl AttackData {
    /// Light attack frame data: 6f startup, 2f active, 10f recovery
    pub fn light() -> Self {
        Self {
            attack_type: AttackType::Light,
            startup_frames: 6,
            active_frames: 2,
            recovery_frames: 10,
            damage: 1,
            on_block: -2,
        }
    }

    /// Heavy attack frame data: 14f startup, 4f active, 18f recovery
    pub fn heavy() -> Self {
        Self {
            attack_type: AttackType::Heavy,
            startup_frames: 14,
            active_frames: 4,
            recovery_frames: 18,
            damage: 2,
            on_block: -8,
        }
    }

    /// Grab frame data: 10f startup, 2f active, 20f recovery (whiff)
    pub fn grab() -> Self {
        Self {
            attack_type: AttackType::Grab,
            startup_frames: 10,
            active_frames: 2,
            recovery_frames: 20,
            damage: 0, // Grab doesn't deal direct damage
            on_block: 0,
        }
    }

    /// Get the total frame duration based on current phase
    pub fn phase_duration(&self, phase: AttackPhase) -> u32 {
        match phase {
            AttackPhase::Startup => self.startup_frames,
            AttackPhase::Active => self.active_frames,
            AttackPhase::Recovery => self.recovery_frames,
        }
    }
}

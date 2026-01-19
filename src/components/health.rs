use bevy::prelude::*;

/// Character health states that modify gameplay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthState {
    /// 100-75% health - Full power, no restrictions
    Whole,
    /// 75-50% health - First blood, minor penalties
    Cut,
    /// 50-25% health - Significant damage, clear disadvantage
    Wounded,
    /// 25-0% health - Critical state, vulnerable to Decisive Blow
    Broken,
}

impl HealthState {
    /// Get the color associated with this health state
    pub fn color(&self) -> Color {
        match self {
            HealthState::Whole => Color::srgb(1.0, 1.0, 1.0),   // White
            HealthState::Cut => Color::srgb(1.0, 0.9, 0.6),      // Light yellow
            HealthState::Wounded => Color::srgb(1.0, 0.6, 0.3),  // Orange
            HealthState::Broken => Color::srgb(0.9, 0.2, 0.2),   // Red
        }
    }

    /// Get movement speed multiplier for this state
    pub fn movement_speed_multiplier(&self) -> f32 {
        match self {
            HealthState::Whole => 1.0,
            HealthState::Cut => 0.95,   // -5%
            HealthState::Wounded => 0.90, // -10%
            HealthState::Broken => 0.85,  // -15%
        }
    }

    /// Get frame advantage penalty for this state
    pub fn frame_advantage_penalty(&self) -> i32 {
        match self {
            HealthState::Whole => 0,
            HealthState::Cut => 0,
            HealthState::Wounded => -2,
            HealthState::Broken => -4,
        }
    }

    /// Get guard meter fill rate multiplier for this state
    pub fn guard_fill_multiplier(&self) -> f32 {
        match self {
            HealthState::Whole => 1.0,
            HealthState::Cut => 1.1,   // +10%
            HealthState::Wounded => 1.2, // +20%
            HealthState::Broken => 1.3,  // +30%
        }
    }

    /// Get pressure decay rate multiplier for this state
    pub fn pressure_decay_multiplier(&self) -> f32 {
        match self {
            HealthState::Whole => 1.0,
            HealthState::Cut => 1.2,   // +20% faster decay
            HealthState::Wounded => 1.5, // +50% faster decay
            HealthState::Broken => 2.0,  // +100% faster decay
        }
    }

    /// Get maximum pressure intensity allowed in this state
    pub fn max_pressure_intensity(&self) -> u8 {
        match self {
            HealthState::Whole => 3,
            HealthState::Cut => 3,
            HealthState::Wounded => 2, // Capped at level 2
            HealthState::Broken => 0,  // Cannot build pressure
        }
    }

    /// Get parry window frames for this state
    pub fn parry_window_frames(&self) -> u32 {
        match self {
            HealthState::Whole => 6,
            HealthState::Cut => 6,
            HealthState::Wounded => 4, // Reduced window
            HealthState::Broken => 3,  // Very tight window
        }
    }

    /// Can this state build momentum?
    pub fn can_build_momentum(&self) -> bool {
        match self {
            HealthState::Broken => false,
            _ => true,
        }
    }
}

/// Character health component
#[derive(Component, Debug)]
pub struct Health {
    /// Current health points (0.0 - 100.0)
    pub current: f32,
    /// Maximum health points
    pub max: f32,
    /// Current health state
    pub state: HealthState,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
            state: HealthState::Whole,
        }
    }

    /// Update health state based on current health percentage
    pub fn update_state(&mut self) {
        let percentage = self.current / self.max;

        self.state = if percentage > 0.75 {
            HealthState::Whole
        } else if percentage > 0.50 {
            HealthState::Cut
        } else if percentage > 0.25 {
            HealthState::Wounded
        } else {
            HealthState::Broken
        };
    }

    /// Apply damage to health
    pub fn take_damage(&mut self, damage: f32) {
        self.current = (self.current - damage).max(0.0);
        self.update_state();
    }

    /// Restore health (for round resets)
    pub fn restore_full(&mut self) {
        self.current = self.max;
        self.state = HealthState::Whole;
    }

    /// Get health percentage (0.0 - 1.0)
    pub fn percentage(&self) -> f32 {
        self.current / self.max
    }

    /// Is this character in critical state?
    pub fn is_broken(&self) -> bool {
        self.state == HealthState::Broken
    }

    /// Is this character healthy enough to perform decisive blow?
    pub fn can_decisive_blow(&self) -> bool {
        matches!(self.state, HealthState::Whole | HealthState::Cut)
    }
}

impl Default for Health {
    fn default() -> Self {
        Self::new(100.0)
    }
}

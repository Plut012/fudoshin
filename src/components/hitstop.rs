use bevy::prelude::*;

/// Component that freezes an entity during hit impact
/// Applied to both attacker and defender when an attack connects
#[derive(Component, Debug)]
pub struct Hitstop {
    /// Frames remaining in freeze
    pub frames_remaining: u32,
    /// Total frames of hitstop (for visual effects)
    pub total_frames: u32,
}

impl Hitstop {
    pub fn new(frames: u32) -> Self {
        Self {
            frames_remaining: frames,
            total_frames: frames,
        }
    }

    /// Tick down the hitstop timer
    pub fn tick(&mut self) -> bool {
        if self.frames_remaining > 0 {
            self.frames_remaining -= 1;
            true
        } else {
            false
        }
    }

    /// Check if hitstop is complete
    pub fn is_complete(&self) -> bool {
        self.frames_remaining == 0
    }

    /// Get progress ratio (0.0 = just started, 1.0 = complete)
    pub fn progress(&self) -> f32 {
        if self.total_frames == 0 {
            1.0
        } else {
            1.0 - (self.frames_remaining as f32 / self.total_frames as f32)
        }
    }
}

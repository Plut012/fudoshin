use bevy::prelude::*;

/// Initiative tracks frame advantage/disadvantage
/// Positive = advantage (can act first), Negative = disadvantage (must wait)
#[derive(Component, Debug)]
pub struct Initiative {
    /// Current frame advantage
    /// Positive = can act first, Negative = must wait
    pub frames: i32,
}

impl Initiative {
    pub fn new() -> Self {
        Self { frames: 0 }
    }

    /// Grant frame advantage
    pub fn gain(&mut self, frames: i32) {
        self.frames = frames;
    }

    /// Set frame disadvantage
    pub fn lose(&mut self, frames: i32) {
        self.frames = -frames;
    }

    /// Check if player has advantage (can act first)
    pub fn has_advantage(&self) -> bool {
        self.frames > 0
    }

    /// Check if player has disadvantage (must wait)
    pub fn has_disadvantage(&self) -> bool {
        self.frames < 0
    }

    /// Tick down initiative by 1 frame
    pub fn tick(&mut self) {
        if self.frames > 0 {
            self.frames -= 1;
        } else if self.frames < 0 {
            self.frames += 1;
        }
    }

    /// Reset to neutral
    pub fn reset(&mut self) {
        self.frames = 0;
    }
}

impl Default for Initiative {
    fn default() -> Self {
        Self::new()
    }
}

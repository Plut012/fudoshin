use bevy::prelude::*;

/// Guard meter - fills when blocking attacks, breaks when full
#[derive(Component, Debug)]
pub struct GuardMeter {
    /// Current guard meter value (0.0 to 1.0)
    pub current: f32,
    /// Maximum guard meter value (normally 1.0)
    pub max: f32,
}

impl GuardMeter {
    pub fn new() -> Self {
        Self {
            current: 0.0,
            max: 1.0,
        }
    }

    /// Add to guard meter (when blocking)
    pub fn fill(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    /// Reduce guard meter (passive drain)
    pub fn drain(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    /// Check if guard is broken
    pub fn is_broken(&self) -> bool {
        self.current >= self.max
    }

    /// Reset guard meter to zero
    pub fn reset(&mut self) {
        self.current = 0.0;
    }
}

impl Default for GuardMeter {
    fn default() -> Self {
        Self::new()
    }
}

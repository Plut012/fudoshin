use bevy::prelude::*;

/// Character state machine
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum CharacterState {
    Idle,
    Walking,
    Stepping,
    Backdashing,
    // More states added in Phase 2
}

impl Default for CharacterState {
    fn default() -> Self {
        Self::Idle
    }
}

/// Frame timer for state transitions
#[derive(Component)]
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

use bevy::prelude::*;

/// Breath (stocks/lives) component - tracks remaining rounds a player can lose
#[derive(Component, Debug)]
pub struct Breath {
    /// Current breaths remaining (0-3)
    pub current: u8,
    /// Maximum breaths
    pub max: u8,
}

impl Breath {
    pub fn new(max: u8) -> Self {
        Self {
            current: max,
            max,
        }
    }

    /// Lose one breath
    pub fn lose_breath(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        }
    }

    /// Has this player lost all breaths?
    pub fn is_defeated(&self) -> bool {
        self.current == 0
    }

    /// Reset breaths to max (for new match)
    pub fn reset(&mut self) {
        self.current = self.max;
    }
}

impl Default for Breath {
    fn default() -> Self {
        Self::new(3)
    }
}

/// Match state resource - tracks round/match state
#[derive(Resource, Debug)]
pub struct MatchState {
    /// Current round number (1-based)
    pub round_number: u32,
    /// Time remaining in current round (seconds)
    pub round_time: f32,
    /// Maximum round time (60 seconds)
    pub max_round_time: f32,
    /// Is the round currently active?
    pub round_active: bool,
    /// Countdown before round starts (3, 2, 1, 0 = fight!)
    pub countdown: f32,
    /// Is match over?
    pub match_over: bool,
    /// Winner entity (if match is over)
    pub winner: Option<Entity>,
}

impl MatchState {
    pub fn new() -> Self {
        Self {
            round_number: 1,
            round_time: 60.0,
            max_round_time: 60.0,
            round_active: false,
            countdown: 3.0,
            match_over: false,
            winner: None,
        }
    }

    /// Start a new round
    pub fn start_round(&mut self) {
        self.round_active = true;
        self.round_time = self.max_round_time;
        self.countdown = 0.0;
    }

    /// End the current round
    pub fn end_round(&mut self) {
        self.round_active = false;
        self.round_number += 1;
        self.countdown = 3.0;
    }

    /// End the match with a winner
    pub fn end_match(&mut self, winner: Entity) {
        self.round_active = false;
        self.match_over = true;
        self.winner = Some(winner);
    }

    /// Is round time expired?
    pub fn is_timeout(&self) -> bool {
        self.round_time <= 0.0
    }

    /// Reset for new match
    pub fn reset(&mut self) {
        self.round_number = 1;
        self.round_time = self.max_round_time;
        self.round_active = false;
        self.countdown = 3.0;
        self.match_over = false;
        self.winner = None;
    }
}

impl Default for MatchState {
    fn default() -> Self {
        Self::new()
    }
}

/// Round end reason for visual feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundEndReason {
    /// Decisive blow landed (Heavy/Grab on Broken opponent)
    DecisiveBlow,
    /// Round timer expired
    Timeout,
}

/// Event fired when a round ends
#[derive(Event, Debug)]
pub struct RoundEndEvent {
    /// Entity that won the round
    pub winner: Entity,
    /// Reason the round ended
    pub reason: RoundEndReason,
}

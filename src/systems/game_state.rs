use bevy::prelude::*;

/// Game states - controls which systems run
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    CharacterSelect,
    InGame,
    Victory,
}

/// Resource to track selected characters
#[derive(Resource, Debug, Default)]
pub struct CharacterSelection {
    pub player1_ready: bool,
    pub player2_ready: bool,
}

impl CharacterSelection {
    pub fn both_ready(&self) -> bool {
        self.player1_ready && self.player2_ready
    }

    pub fn reset(&mut self) {
        self.player1_ready = false;
        self.player2_ready = false;
    }
}

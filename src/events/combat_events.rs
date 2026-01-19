use bevy::prelude::*;

/// Event fired when an attack hits a hurtbox
#[derive(Event, Debug, Clone)]
pub struct HitEvent {
    /// Entity that performed the attack
    pub attacker: Entity,
    /// Entity that was hit
    pub defender: Entity,
    /// Amount of damage
    pub damage: u8,
    /// Whether the hit was blocked
    pub was_blocked: bool,
    /// Properties of the attack
    pub unblockable: bool,
}

impl HitEvent {
    pub fn new(attacker: Entity, defender: Entity, damage: u8) -> Self {
        Self {
            attacker,
            defender,
            damage,
            was_blocked: false,
            unblockable: false,
        }
    }

    pub fn blocked(mut self) -> Self {
        self.was_blocked = true;
        self
    }

    pub fn unblockable(mut self) -> Self {
        self.unblockable = true;
        self
    }
}

/// Event fired when a parry successfully deflects an attack
#[derive(Event, Debug)]
pub struct ParryEvent {
    /// Entity that performed the parry
    pub defender: Entity,
    /// Entity whose attack was parried
    pub attacker: Entity,
}

/// Event fired when guard meter fills and breaks
#[derive(Event, Debug)]
pub struct GuardBreakEvent {
    /// Entity whose guard broke
    pub entity: Entity,
}

/// Event fired when a grab connects
#[derive(Event, Debug)]
pub struct GrabEvent {
    /// Entity that performed the grab
    pub attacker: Entity,
    /// Entity that was grabbed
    pub defender: Entity,
}

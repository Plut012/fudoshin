use bevy::prelude::*;

/// Properties that modify attack behavior
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttackProperty {
    /// Attack absorbs one light hit during startup (Heavy attacks)
    LightArmor,
    /// Cannot be blocked (Grabs)
    Unblockable,
    /// Can chain into another attack
    Chainable,
}

/// Offensive hitbox - damages opponents when active
#[derive(Component, Debug)]
pub struct Hitbox {
    /// Rectangle offset from entity position (local space)
    pub rect: Rect,
    /// Whether this hitbox is currently active
    pub active: bool,
    /// Amount of damage (in health states)
    pub damage: u8,
    /// Special properties of this attack
    pub properties: Vec<AttackProperty>,
}

impl Hitbox {
    pub fn new(rect: Rect, damage: u8) -> Self {
        Self {
            rect,
            active: false,
            damage,
            properties: vec![],
        }
    }

    pub fn with_properties(mut self, properties: Vec<AttackProperty>) -> Self {
        self.properties = properties;
        self
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Get world-space rect from entity transform
    pub fn world_rect(&self, transform: &Transform) -> Rect {
        Rect {
            min: Vec2::new(
                transform.translation.x + self.rect.min.x,
                transform.translation.y + self.rect.min.y,
            ),
            max: Vec2::new(
                transform.translation.x + self.rect.max.x,
                transform.translation.y + self.rect.max.y,
            ),
        }
    }
}

/// Defensive hurtbox - receives damage when hit
#[derive(Component, Debug)]
pub struct Hurtbox {
    /// Rectangle offset from entity position (local space)
    pub rect: Rect,
}

impl Hurtbox {
    pub fn new(rect: Rect) -> Self {
        Self { rect }
    }

    /// Get world-space rect from entity transform
    pub fn world_rect(&self, transform: &Transform) -> Rect {
        Rect {
            min: Vec2::new(
                transform.translation.x + self.rect.min.x,
                transform.translation.y + self.rect.min.y,
            ),
            max: Vec2::new(
                transform.translation.x + self.rect.max.x,
                transform.translation.y + self.rect.max.y,
            ),
        }
    }
}

impl Default for Hurtbox {
    fn default() -> Self {
        // Default hurtbox for character body
        Self::new(Rect::from_center_size(Vec2::ZERO, Vec2::new(60.0, 120.0)))
    }
}

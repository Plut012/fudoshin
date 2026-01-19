use bevy::prelude::*;
use crate::components::character::Player;
use crate::events::combat_events::{HitEvent, ParryEvent};

/// Component to track momentum (win streak)
#[derive(Component, Debug)]
pub struct Momentum {
    /// Current momentum level (0-5)
    pub level: u8,
    /// Frames since last successful action
    pub frames_since_action: u32,
    /// Frames before momentum starts decaying (120f = 2 seconds)
    pub decay_threshold: u32,
}

impl Momentum {
    pub fn new() -> Self {
        Self {
            level: 0,
            frames_since_action: 0,
            decay_threshold: 120,
        }
    }

    /// Gain momentum from successful action
    pub fn gain(&mut self) {
        self.level = (self.level + 1).min(5);
        self.frames_since_action = 0;
    }

    /// Lose momentum from being hit or missing
    pub fn lose(&mut self) {
        self.level = self.level.saturating_sub(1);
        self.frames_since_action = 0;
    }

    /// Reset momentum completely
    pub fn reset(&mut self) {
        self.level = 0;
        self.frames_since_action = 0;
    }

    /// Get damage bonus percentage (level 3+)
    pub fn damage_bonus(&self) -> f32 {
        match self.level {
            0..=2 => 1.0,
            3 => 1.1,      // +10% damage
            4 => 1.15,     // +15% damage
            5 => 1.25,     // +25% damage
            _ => 1.0,
        }
    }

    /// Get guard damage bonus (level 3+)
    pub fn guard_damage_bonus(&self) -> f32 {
        match self.level {
            0..=2 => 1.0,
            3 => 1.2,      // +20% guard damage
            4 => 1.3,      // +30% guard damage
            5 => 1.5,      // +50% guard damage
            _ => 1.0,
        }
    }

    /// Tick momentum decay timer
    pub fn tick(&mut self) {
        self.frames_since_action += 1;

        // Start decaying after threshold
        if self.frames_since_action > self.decay_threshold {
            // Decay every 60 frames (1 second)
            let decay_interval = 60;
            let frames_over = self.frames_since_action - self.decay_threshold;
            if frames_over % decay_interval == 0 && self.level > 0 {
                self.level -= 1;
            }
        }
    }
}

impl Default for Momentum {
    fn default() -> Self {
        Self::new()
    }
}

/// Build momentum on successful hits
pub fn build_momentum_on_hit(
    mut hit_events: EventReader<HitEvent>,
    mut query: Query<&mut Momentum>,
) {
    for event in hit_events.read() {
        // Attacker gains momentum
        if let Ok(mut momentum) = query.get_mut(event.attacker) {
            momentum.gain();
            info!("Player gained momentum! Level: {}", momentum.level);
        }

        // Defender loses momentum if hit (not blocked)
        if !event.was_blocked {
            if let Ok(mut momentum) = query.get_mut(event.defender) {
                momentum.lose();
                if momentum.level > 0 {
                    info!("Player lost momentum! Level: {}", momentum.level);
                }
            }
        }
    }
}

/// Build momentum on successful parries
pub fn build_momentum_on_parry(
    mut parry_events: EventReader<ParryEvent>,
    mut query: Query<&mut Momentum>,
) {
    for event in parry_events.read() {
        // Defender (parrier) gains extra momentum
        if let Ok(mut momentum) = query.get_mut(event.defender) {
            momentum.gain();
            momentum.gain(); // Double gain for successful parry
            info!("Player gained momentum from parry! Level: {}", momentum.level);
        }

        // Attacker loses momentum
        if let Ok(mut momentum) = query.get_mut(event.attacker) {
            momentum.lose();
        }
    }
}

/// Tick momentum decay timer
pub fn tick_momentum(
    mut query: Query<&mut Momentum>,
) {
    for mut momentum in query.iter_mut() {
        momentum.tick();
    }
}

/// Visual feedback for momentum levels (subtle aura/glow)
pub fn visualize_momentum(
    mut gizmos: Gizmos,
    query: Query<(&Momentum, &Transform, &Player)>,
) {
    for (momentum, transform, _player) in query.iter() {
        if momentum.level >= 3 {
            let pos = transform.translation.truncate();

            // Draw expanding rings for momentum levels 3+
            let base_radius = 50.0;
            let color = match momentum.level {
                3 => Color::srgb(0.3, 1.0, 0.3),  // Green
                4 => Color::srgb(0.3, 0.8, 1.0),  // Cyan
                5 => Color::srgb(1.0, 0.8, 0.0),  // Gold
                _ => Color::srgb(0.5, 0.5, 0.5),  // Gray
            };

            // Multiple rings for visual effect
            for i in 0..momentum.level - 2 {
                let radius = base_radius + (i as f32 * 15.0);
                gizmos.circle_2d(pos, radius, color);
            }
        }
    }
}

/// Debug: Log momentum changes
pub fn debug_momentum(
    query: Query<(&Player, &Momentum), Changed<Momentum>>,
) {
    for (player, momentum) in query.iter() {
        if momentum.level > 0 || momentum.frames_since_action == 0 {
            debug!(
                "Player {:?} momentum: level={}, frames_since_action={}",
                player, momentum.level, momentum.frames_since_action
            );
        }
    }
}

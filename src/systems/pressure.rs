use bevy::prelude::*;
use crate::components::character::{MaxSpeed, Player};
use crate::components::initiative::Initiative;
use crate::events::combat_events::HitEvent;

/// Pressure state - tracks offensive momentum
#[derive(Component, Debug)]
pub struct Pressure {
    /// Intensity level (0-3)
    /// 0 = None, 1 = Light, 2 = Medium, 3 = Heavy
    pub intensity: u8,
}

impl Pressure {
    pub fn new() -> Self {
        Self { intensity: 0 }
    }

    /// Increase pressure (max 3)
    pub fn increase(&mut self) {
        self.intensity = (self.intensity + 1).min(3);
    }

    /// Decrease pressure (min 0)
    pub fn decrease(&mut self) {
        self.intensity = self.intensity.saturating_sub(1);
    }

    /// Reset to no pressure
    pub fn reset(&mut self) {
        self.intensity = 0;
    }

    /// Get movement speed multiplier
    pub fn movement_bonus(&self) -> f32 {
        match self.intensity {
            0 => 1.0,    // No bonus
            1 => 1.05,   // +5%
            2 => 1.10,   // +10%
            3 => 1.15,   // +15%
            _ => 1.0,
        }
    }

    /// Get attack speed bonus (frames reduced from startup)
    pub fn attack_speed_bonus(&self) -> u32 {
        match self.intensity {
            0 => 0,  // No bonus
            1 => 0,  // Light pressure: no attack bonus yet
            2 => 1,  // Medium pressure: -1f startup
            3 => 2,  // Heavy pressure: -2f startup
            _ => 0,
        }
    }
}

impl Default for Pressure {
    fn default() -> Self {
        Self::new()
    }
}

/// Build pressure when landing hits while having advantage
pub fn build_pressure(
    mut hit_events: EventReader<HitEvent>,
    mut query: Query<(&Initiative, &mut Pressure, &Player)>,
) {
    for event in hit_events.read() {
        // Attacker builds pressure if they land a hit
        if let Ok((initiative, mut pressure, player)) = query.get_mut(event.attacker) {
            // Only build pressure if:
            // 1. Hit landed (not blocked), OR
            // 2. Hit was blocked but attacker has advantage
            let should_build = !event.was_blocked || initiative.has_advantage();

            if should_build {
                pressure.increase();
                info!(
                    "Player {:?} building pressure! Level: {}",
                    player, pressure.intensity
                );
            }
        }

        // Defender loses pressure when hit
        if let Ok((_, mut pressure, player)) = query.get_mut(event.defender) {
            if !event.was_blocked && pressure.intensity > 0 {
                pressure.reset();
                info!("Player {:?} lost pressure (got hit)", player);
            }
        }
    }
}

/// Apply pressure bonuses to movement speed
pub fn apply_pressure_movement_bonus(
    mut query: Query<(&Pressure, &mut MaxSpeed), Changed<Pressure>>,
) {
    for (pressure, mut max_speed) in query.iter_mut() {
        // Base speed is 300.0, apply multiplier
        let base_speed = 300.0;
        max_speed.0 = base_speed * pressure.movement_bonus();
    }
}

/// Gradually drain pressure when not actively attacking
pub fn drain_pressure_passive(
    time: Res<Time>,
    mut query: Query<(&mut Pressure, &crate::components::state::CharacterState)>,
) {
    for (mut pressure, state) in query.iter_mut() {
        // Only drain when idle (not actively fighting)
        if matches!(state, crate::components::state::CharacterState::Idle) {
            // Drain slowly over time (every ~2 seconds at 60fps)
            // Using a simple frame counter approach
            static mut FRAME_COUNTER: u32 = 0;
            unsafe {
                FRAME_COUNTER += 1;
                if FRAME_COUNTER >= 120 && pressure.intensity > 0 {
                    pressure.decrease();
                    FRAME_COUNTER = 0;
                    debug!("Pressure drained (idle), now at level {}", pressure.intensity);
                }
            }
        }
    }
}

/// Visual feedback for pressure (glow effect)
pub fn visualize_pressure(
    mut query: Query<(&Pressure, &mut Sprite, &Player)>,
) {
    for (pressure, mut sprite, player) in query.iter_mut() {
        // Base colors
        let base_color = match player {
            Player::One => Color::srgb(0.9, 0.2, 0.2),   // Red
            Player::Two => Color::srgb(0.2, 0.4, 0.9),   // Blue
        };

        // Brighten based on pressure intensity
        let brightness_boost = match pressure.intensity {
            0 => 1.0,   // Normal
            1 => 1.1,   // +10% brightness
            2 => 1.25,  // +25% brightness
            3 => 1.5,   // +50% brightness (very bright)
            _ => 1.0,
        };

        sprite.color = Color::srgb(
            (base_color.to_srgba().red * brightness_boost).min(1.0),
            (base_color.to_srgba().green * brightness_boost).min(1.0),
            (base_color.to_srgba().blue * brightness_boost).min(1.0),
        );
    }
}

/// Debug: Log pressure changes
pub fn debug_pressure(
    query: Query<(&Player, &Pressure), Changed<Pressure>>,
) {
    for (player, pressure) in query.iter() {
        debug!(
            "Player {:?} pressure: {} (speed: +{}%, attack: -{}f)",
            player,
            pressure.intensity,
            ((pressure.movement_bonus() - 1.0) * 100.0) as i32,
            pressure.attack_speed_bonus()
        );
    }
}

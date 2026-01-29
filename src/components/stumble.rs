use bevy::prelude::*;

/// Direction of stumble/tumble
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StumbleDirection {
    /// Stumble backward (away from attacker)
    Backward,
    /// Stumble forward (toward attacker)
    Forward,
    /// Stumble down (low tumble)
    Down,
}

/// Stumble state - defender is off-balance and vulnerable but can act
///
/// This creates a juggle-like mechanic where:
/// - Defender can tech out (8-frame window)
/// - Attacker can extend with directional hits
/// - Wall bounces create big opportunities
/// - Spike finishers end the juggle
#[derive(Component, Debug)]
pub struct StumbleState {
    /// Total frames of stumble (includes extensions)
    pub frames_remaining: u32,

    /// Direction of stumble
    pub direction: StumbleDirection,

    /// When tech window opens (frame number)
    pub tech_window_start: u32,

    /// When tech window closes (frame number)
    pub tech_window_end: u32,

    /// Can this stumble be teched?
    pub can_tech: bool,

    /// Number of times this stumble has been extended
    pub extension_count: u8,

    /// Was this stumble caused by counter hit?
    pub from_counter_hit: bool,

    /// Frame counter for tech window calculation
    pub elapsed_frames: u32,
}

impl StumbleState {
    /// Create a new stumble state
    pub fn new(direction: StumbleDirection, duration: u32, can_tech: bool) -> Self {
        Self {
            frames_remaining: duration,
            direction,
            tech_window_start: 5,   // Tech window opens at frame 5
            tech_window_end: 13,     // Tech window closes at frame 13 (8f window)
            can_tech,
            extension_count: 0,
            from_counter_hit: false,
            elapsed_frames: 0,
        }
    }

    /// Check if currently in tech window
    pub fn is_in_tech_window(&self) -> bool {
        self.can_tech
            && self.elapsed_frames >= self.tech_window_start
            && self.elapsed_frames <= self.tech_window_end
    }

    /// Tick the stumble state (call each frame)
    pub fn tick(&mut self) {
        self.elapsed_frames += 1;
        if self.frames_remaining > 0 {
            self.frames_remaining -= 1;
        }
    }

    /// Extend the stumble with additional frames (diminishing returns)
    pub fn extend(&mut self, direction: StumbleDirection, base_frames: u32) {
        // Diminishing returns on extensions
        let scaled_frames = match self.extension_count {
            0 => base_frames,                    // 100% - 15f
            1 => (base_frames * 80) / 100,       // 80% - 12f
            2 => (base_frames * 66) / 100,       // 66% - 10f
            3 => (base_frames * 53) / 100,       // 53% - 8f
            _ => 0,  // Max 4 extensions
        };

        if scaled_frames > 0 {
            self.frames_remaining += scaled_frames;
            self.direction = direction;
            self.extension_count += 1;

            // Reset tech window for new extension
            self.elapsed_frames = 0;
            self.tech_window_start = 5;
            self.tech_window_end = 13;

            debug!(
                "Stumble extended! Direction: {:?}, Extension #{}, Added {}f",
                direction, self.extension_count, scaled_frames
            );
        }
    }

    /// Check if stumble should end (ran out of time or max extensions)
    pub fn should_end(&self) -> bool {
        self.frames_remaining == 0 || self.extension_count >= 4
    }

    /// Apply wall bounce (reverses direction, adds time, prevents tech)
    pub fn apply_wall_bounce(&mut self) {
        // Reverse direction
        self.direction = match self.direction {
            StumbleDirection::Backward => StumbleDirection::Forward,
            StumbleDirection::Forward => StumbleDirection::Backward,
            d => d,  // Down doesn't bounce
        };

        // Add extra vulnerability
        self.frames_remaining += 20;

        // Cannot tech during wall bounce
        self.can_tech = false;

        // Reset elapsed for visual feedback
        self.elapsed_frames = 0;

        info!("WALL BOUNCE! Direction: {:?}, +20f, NO TECH", self.direction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stumble_creation() {
        let stumble = StumbleState::new(StumbleDirection::Backward, 30, true);
        assert_eq!(stumble.frames_remaining, 30);
        assert_eq!(stumble.direction, StumbleDirection::Backward);
        assert!(stumble.can_tech);
        assert_eq!(stumble.extension_count, 0);
    }

    #[test]
    fn test_tech_window() {
        let mut stumble = StumbleState::new(StumbleDirection::Backward, 30, true);

        // Before window
        assert!(!stumble.is_in_tech_window());

        // Advance to window
        for _ in 0..5 {
            stumble.tick();
        }
        assert!(stumble.is_in_tech_window());

        // Past window
        for _ in 0..10 {
            stumble.tick();
        }
        assert!(!stumble.is_in_tech_window());
    }

    #[test]
    fn test_extension_diminishing_returns() {
        let mut stumble = StumbleState::new(StumbleDirection::Backward, 30, true);

        // First extension: 100%
        stumble.extend(StumbleDirection::Forward, 15);
        assert_eq!(stumble.extension_count, 1);

        // Second extension: 80%
        stumble.extend(StumbleDirection::Backward, 15);
        assert_eq!(stumble.extension_count, 2);

        // Third extension: 66%
        stumble.extend(StumbleDirection::Forward, 15);
        assert_eq!(stumble.extension_count, 3);

        // Fourth extension: 53%
        stumble.extend(StumbleDirection::Backward, 15);
        assert_eq!(stumble.extension_count, 4);

        // Fifth extension: blocked (max 4)
        let count_before = stumble.extension_count;
        stumble.extend(StumbleDirection::Forward, 15);
        assert_eq!(stumble.extension_count, count_before);
    }

    #[test]
    fn test_wall_bounce() {
        let mut stumble = StumbleState::new(StumbleDirection::Backward, 30, true);

        stumble.apply_wall_bounce();

        // Direction reversed
        assert_eq!(stumble.direction, StumbleDirection::Forward);

        // Extra time added
        assert_eq!(stumble.frames_remaining, 50); // 30 + 20

        // Cannot tech
        assert!(!stumble.can_tech);
    }
}

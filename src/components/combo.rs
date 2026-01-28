use bevy::prelude::*;
use crate::components::state::AttackType;

/// Buffers recent button presses for lenient combo execution
///
/// This component provides an 8-frame buffer window for inputs,
/// making combo execution feel responsive and forgiving rather
/// than requiring frame-perfect timing.
#[derive(Component, Debug, Clone)]
pub struct InputBuffer {
    /// Frames since Light attack was pressed (0 = not buffered)
    /// Values 1-8 indicate the input is buffered
    pub light: u8,

    /// Frames since Heavy attack was pressed
    pub heavy: u8,

    /// Frames since Grab was pressed
    pub grab: u8,

    /// Buffer window in frames (default: 8)
    pub window: u8,
}

impl InputBuffer {
    /// Create a new input buffer with 8-frame window
    pub fn new() -> Self {
        Self {
            light: 0,
            heavy: 0,
            grab: 0,
            window: 8,
        }
    }

    /// Record a button press (sets timer to 1)
    ///
    /// This marks an input as "just pressed" by setting its timer to 1.
    /// The timer will increment each frame until it exceeds the buffer window.
    pub fn press(&mut self, attack_type: AttackType) {
        match attack_type {
            AttackType::Light => self.light = 1,
            AttackType::Heavy => self.heavy = 1,
            AttackType::Grab => self.grab = 1,
        }
    }

    /// Age all buffers by 1 frame
    ///
    /// Call this every frame to tick down buffers. Inputs older than
    /// the buffer window are automatically cleared.
    pub fn tick(&mut self) {
        // Increment active buffers
        if self.light > 0 {
            self.light += 1;
        }
        if self.heavy > 0 {
            self.heavy += 1;
        }
        if self.grab > 0 {
            self.grab += 1;
        }

        // Clear buffers that exceed window
        if self.light > self.window {
            self.light = 0;
        }
        if self.heavy > self.window {
            self.heavy = 0;
        }
        if self.grab > self.window {
            self.grab = 0;
        }
    }

    /// Check if an attack type is currently buffered
    ///
    /// Returns true if the input was pressed within the last 8 frames
    pub fn is_buffered(&self, attack_type: AttackType) -> bool {
        let frames = match attack_type {
            AttackType::Light => self.light,
            AttackType::Heavy => self.heavy,
            AttackType::Grab => self.grab,
        };
        frames > 0 && frames <= self.window
    }

    /// Consume a buffered input (clears it)
    ///
    /// Call this when an input is used for a cancel to prevent
    /// it from being consumed multiple times
    pub fn consume(&mut self, attack_type: AttackType) {
        match attack_type {
            AttackType::Light => self.light = 0,
            AttackType::Heavy => self.heavy = 0,
            AttackType::Grab => self.grab = 0,
        }
    }

    /// Check how many frames ago an input was pressed
    ///
    /// Returns 0 if not buffered, otherwise returns frame count
    pub fn frames_ago(&self, attack_type: AttackType) -> u8 {
        match attack_type {
            AttackType::Light => self.light,
            AttackType::Heavy => self.heavy,
            AttackType::Grab => self.grab,
        }
    }

    /// Clear all buffered inputs
    pub fn clear(&mut self) {
        self.light = 0;
        self.heavy = 0;
        self.grab = 0;
    }
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_buffer_press() {
        let mut buffer = InputBuffer::new();

        buffer.press(AttackType::Light);
        assert_eq!(buffer.light, 1);
        assert!(buffer.is_buffered(AttackType::Light));
    }

    #[test]
    fn test_input_buffer_aging() {
        let mut buffer = InputBuffer::new();

        buffer.press(AttackType::Light);
        assert_eq!(buffer.light, 1);

        buffer.tick();
        assert_eq!(buffer.light, 2);
        assert!(buffer.is_buffered(AttackType::Light));

        // Age beyond window
        for _ in 0..7 {
            buffer.tick();
        }
        assert!(!buffer.is_buffered(AttackType::Light));
        assert_eq!(buffer.light, 0);
    }

    #[test]
    fn test_input_buffer_consume() {
        let mut buffer = InputBuffer::new();

        buffer.press(AttackType::Heavy);
        assert!(buffer.is_buffered(AttackType::Heavy));

        buffer.consume(AttackType::Heavy);
        assert!(!buffer.is_buffered(AttackType::Heavy));
    }

    #[test]
    fn test_multiple_inputs() {
        let mut buffer = InputBuffer::new();

        buffer.press(AttackType::Light);
        buffer.press(AttackType::Heavy);

        assert!(buffer.is_buffered(AttackType::Light));
        assert!(buffer.is_buffered(AttackType::Heavy));
        assert!(!buffer.is_buffered(AttackType::Grab));
    }
}

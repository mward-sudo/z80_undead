use std::time::Duration;

/// Standard Z80 clock frequency in Hz
pub const Z80_CLOCK_FREQUENCY: u32 = 4_000_000; // 4MHz

/// RetroArch frame rate (typically 60 FPS)
pub const RETROARCH_FPS: u32 = 60;

/// Represents timing conversion utilities
pub struct TimingConverter {
    clock_frequency: u32,
    t_states_per_frame: u32,
    current_frame_t_states: u32,
}

impl Default for TimingConverter {
    fn default() -> Self {
        Self::new(Z80_CLOCK_FREQUENCY)
    }
}

impl TimingConverter {
    /// Creates a new timing converter with specified clock frequency
    pub fn new(clock_frequency: u32) -> Self {
        let t_states_per_frame = clock_frequency / RETROARCH_FPS;
        Self {
            clock_frequency,
            t_states_per_frame,
            current_frame_t_states: 0,
        }
    }

    /// Returns the number of T-states per frame
    pub fn t_states_per_frame(&self) -> u32 {
        self.t_states_per_frame
    }

    /// Updates frame T-states and checks if frame boundary is reached
    pub fn update_frame_t_states(&mut self, t_states: u32) -> bool {
        self.current_frame_t_states += t_states;
        if self.current_frame_t_states >= self.t_states_per_frame {
            self.current_frame_t_states -= self.t_states_per_frame;
            true
        } else {
            false
        }
    }

    /// Converts RetroArch frames to T-states
    pub fn frames_to_t_states(&self, frames: u32) -> u32 {
        frames * self.t_states_per_frame
    }

    /// Converts T-states to RetroArch frames (rounded down)
    pub fn t_states_to_frames(&self, t_states: u32) -> u32 {
        t_states / self.t_states_per_frame
    }

    /// Returns remaining T-states in current frame
    pub fn remaining_t_states(&self) -> u32 {
        self.t_states_per_frame - self.current_frame_t_states
    }

    /// Sets the clock frequency and updates timing calculations
    pub fn set_clock_frequency(&mut self, frequency: u32) {
        self.clock_frequency = frequency;
        self.t_states_per_frame = frequency / RETROARCH_FPS;
        self.current_frame_t_states = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timing_conversion() {
        let converter = TimingConverter::default();

        // At 4MHz, we expect 66666 T-states per frame (rounded down)
        assert_eq!(converter.t_states_per_frame(), 66666);

        // Test frame to T-state conversion
        assert_eq!(converter.frames_to_t_states(1), 66666);
        assert_eq!(converter.frames_to_t_states(2), 133332);

        // Test T-state to frame conversion
        assert_eq!(converter.t_states_to_frames(66666), 1);
        assert_eq!(converter.t_states_to_frames(133332), 2);
    }

    #[test]
    fn test_frame_boundary_detection() {
        let mut converter = TimingConverter::default();

        // Add T-states just under frame boundary
        assert!(!converter.update_frame_t_states(66665));

        // Add one more T-state to trigger frame boundary
        assert!(converter.update_frame_t_states(1));

        // Check that current_frame_t_states was reset
        assert_eq!(converter.current_frame_t_states, 0);
    }

    #[test]
    fn test_clock_frequency_change() {
        let mut converter = TimingConverter::default();

        // Change to 3.5MHz
        converter.set_clock_frequency(3_500_000);

        // Verify new T-states per frame calculation
        assert_eq!(converter.t_states_per_frame(), 58333);
    }

    #[test]
    fn test_remaining_t_states() {
        let mut converter = TimingConverter::default();

        // Add some T-states
        converter.update_frame_t_states(30000);

        // Check remaining T-states
        assert_eq!(converter.remaining_t_states(), 36666);
    }
}

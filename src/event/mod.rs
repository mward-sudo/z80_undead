//! Event system for handling CPU and system events

/// Represents different types of events in the system
#[derive(Debug, Clone, Copy)]
pub enum Event {
    Interrupt,
    Timer,
    // Add more event types as needed
}

/// Manages event queue and timing
pub struct EventQueue {
    events: Vec<(Event, u32)>, // (event, t_state)
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl EventQueue {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: Event, t_state: u32) {
        self.events.push((event, t_state));
        self.events.sort_by_key(|&(_, t)| t);
    }

    pub fn peek(&self) -> Option<&(Event, u32)> {
        self.events.first()
    }

    pub fn pop(&mut self) -> Option<(Event, u32)> {
        if self.events.is_empty() {
            None
        } else {
            Some(self.events.remove(0))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

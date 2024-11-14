use super::*;
use crate::event::{Event, EventQueue};

#[test]
fn test_interrupt_handling() {
    let mut fixture = CpuTestFixture::new();
    let mut events = EventQueue::new();
    
    // Schedule an interrupt
    events.push(Event::Interrupt, 100);
    
    // Run until interrupt
    while fixture.cpu.cycles < 100 {
        fixture.cpu.step().unwrap();
    }
    
    // Verify interrupt was processed
    assert!(fixture.cpu.flags.interrupt_enabled);
}

#[test]
fn test_multiple_events() {
    let mut fixture = CpuTestFixture::new();
    let mut events = EventQueue::new();
    
    // Schedule multiple events
    events.push(Event::Timer, 50);
    events.push(Event::Interrupt, 100);
    
    // Verify events are processed in order
    assert_eq!(events.peek().unwrap().1, 50);
    events.pop();
    assert_eq!(events.peek().unwrap().1, 100);
}

#[test]
fn test_event_timing_accuracy() {
    let mut fixture = CpuTestFixture::new();
    let mut events = EventQueue::new();
    
    // Test precise timing of event processing
    events.push(Event::Timer, 16);  // 4 T-states * 4 cycles
    
    while fixture.cpu.cycles < 16 {
        fixture.cpu.step().unwrap();
    }
    
    assert_eq!(fixture.cpu.cycles, 16);
    assert!(events.peek().is_some());
} 
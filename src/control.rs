//basic control operations

use std::sync::{Arc, Mutex};
use tracing::info;

// Define shared system state
pub struct SubstationState {
    pub breaker_open: bool,
    pub voltage: f64,
}

// Shared state for the control system
pub fn create_shared_state() -> Arc<Mutex<SubstationState>> {
    Arc::new(Mutex::new(SubstationState {
        breaker_open: false,
        voltage: 230.0,
    }))
}

// Function to open/close circuit breaker
pub fn toggle_breaker(state: Arc<Mutex<SubstationState>>, open: bool) {
    let mut substation = state.lock().unwrap();
    substation.breaker_open = open;
    
    if open {
        info!("⚡ Circuit breaker OPENED. Power flow stopped.");
    } else {
        info!("⚡ Circuit breaker CLOSED. Power restored.");
    }
}

// Function to adjust voltage
pub fn adjust_voltage(state: Arc<Mutex<SubstationState>>, new_voltage: f64) {
    let mut substation = state.lock().unwrap();
    substation.voltage = new_voltage;
    
    info!("🔧 Voltage adjusted to {}V", new_voltage);
}

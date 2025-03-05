use std::time::Duration;
use std::sync::{Arc, Mutex};
use tokio::time::sleep;
use tracing::info;
use crate::control::SubstationState;

pub async fn monitor_substation(state: Arc<Mutex<SubstationState>>) {
    loop {
        let substation = state.lock().unwrap();
        let voltage = substation.voltage;
        let current = 15.0; // Placeholder value for current

        info!("⚡ Voltage: {}V | Current: {}A", voltage, current);

        drop(substation); // Release the lock before sleeping
        sleep(Duration::from_secs(5)).await;
    }
}
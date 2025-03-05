use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

pub async fn monitor_substation() {
    loop {
        let voltage = 230.0; // Assume the voltage is 230V
        let current = 15.0; // Assume the current is 15A
        
        info!("Voltage: {}V | Curent: {}A", voltage, current);
        
        sleep(Duration::from_secs(5)).await;
    }
}
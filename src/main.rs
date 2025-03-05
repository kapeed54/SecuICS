mod monitor;
mod control;

use tokio::runtime::Runtime;
use tracing_subscriber;
use std::io;
//use std::sync::Arc;
use control::{create_shared_state, toggle_breaker, adjust_voltage};

fn main() {
    tracing_subscriber::fmt::init();

    let rt = Runtime::new().unwrap();
    let substation_state = create_shared_state(); // Create shared state

    loop {
        println!("\n🔹 Power Substation Control Menu 🔹");
        println!("[1] Open Circuit Breaker");
        println!("[2] Close Circuit Breaker");
        println!("[3] Adjust Voltage");
        println!("[4] Start Monitoring");
        println!("[5] Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => toggle_breaker(substation_state.clone(), true),
            2 => toggle_breaker(substation_state.clone(), false),
            3 => {
                println!("Enter new voltage: ");
                let mut voltage_input = String::new();
                io::stdin().read_line(&mut voltage_input).expect("Failed to read input");
                match voltage_input.trim().parse::<f64>() {
                    Ok(voltage) => adjust_voltage(substation_state.clone(), voltage),
                    Err(_) => println!("❌ Invalid voltage value."),
                }
            }
            4 => rt.block_on(monitor::monitor_substation(substation_state.clone())), // ✅ FIXED: Passing state
            5 => {
                println!("Exiting...");
                break;
            }
            _ => println!("❌ Invalid choice."),
        }
    }
}

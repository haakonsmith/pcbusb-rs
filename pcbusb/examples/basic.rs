//! Basic example demonstrating CAN communication on macOS using pcbusb
//!
//! This example shows how to:
//! - Initialize a CAN interface
//! - Send a CAN message
//! - Receive CAN messages
//! - Handle basic filtering

use ::nb;
use embedded_can::blocking::Can as BlockingCan;
use embedded_can::nb::Can as NbCan;

use pcbusb::prelude::*;
use pcbusb::{Filter, Frame, Interface, StandardId};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting macOS CAN interface example...");

    // Initialize the CAN interface
    println!("Initializing CAN interface...");
    let mut can_interface = Interface::init(pcbusb::Baudrate::Baud500k)?;
    println!("CAN interface initialized successfully!");

    // Add a filter to accept all messages (optional)
    println!("Setting up message filter...");
    let filter = Filter::accept_all();
    can_interface.add_filter(&filter)?;
    println!("Filter configured to accept all messages");

    // Create a test message to send
    let test_id = StandardId::new(0x123).unwrap();
    let test_data = [0x01, 0x02, 0x03, 0x04];
    let test_frame = Frame::new(test_id, &test_data).unwrap();

    println!("Sending test message with ID 0x{:03X}...", test_id.as_raw());

    // Try to send the message using blocking API
    match BlockingCan::transmit(&mut can_interface, &test_frame) {
        Ok(()) => println!("Test message sent successfully!"),
        Err(e) => println!("Failed to send test message: {}", e),
    }

    // Try to receive messages for a short time
    println!("Listening for CAN messages (5 second timeout)...");
    let start_time = Instant::now();
    let timeout = Duration::from_secs(5);
    let mut message_count = 0;

    while start_time.elapsed() < timeout {
        // Try non-blocking receive using the nb trait
        match NbCan::receive(&mut can_interface) {
            Ok(frame) => {
                message_count += 1;
                println!(
                    "Received message #{}: ID=0x{:03X}, Data={:02X?}",
                    message_count,
                    match frame.id() {
                        pcbusb::Id::Standard(id) => id.as_raw() as u32,
                        pcbusb::Id::Extended(id) => id.as_raw(),
                    },
                    frame.data()
                );
            }
            Err(nb::Error::WouldBlock) => {
                // No message available, continue polling
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(nb::Error::Other(e)) => {
                println!("Error receiving message: {}", e);
                break;
            }
        }
    }

    if message_count == 0 {
        println!("No messages received during the listening period.");
        println!("This is normal if no other CAN devices are connected and transmitting.");
    } else {
        println!("Total messages received: {}", message_count);
    }

    // Demonstrate filtering with a specific ID
    println!("\nSetting up specific ID filter for 0x456...");
    can_interface.clear_filters();
    let specific_id = StandardId::new(0x456).unwrap();
    let specific_filter = Filter::new(specific_id.into());

    match can_interface.add_filter(&specific_filter) {
        Ok(()) => println!("Filter set for ID 0x456"),
        Err(e) => println!("Failed to set specific filter: {}", e),
    }

    println!("Example completed successfully!");
    Ok(())
}

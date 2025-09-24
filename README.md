# pcbusb-rs

Ergonomic wrapper for the MacCAN PCBUSB API for macOS. This library has been converted from Windows-specific code to work natively on macOS using POSIX APIs instead of Windows synchronization primitives.

See `README.md` in the `pcbusb-sys` folder for more details on the raw bindings.

This aims to mimic https://github.com/timokroeger/pcan-basic-rs

## Features

- Safe Rust bindings for MacCAN PCBUSB library
- macOS-exclusive CAN interface support using safe POSIX APIs
- Compatible with PEAK-System PCAN-USB devices on macOS
- Type-safe API wrapper around the raw C bindings
- Non-blocking and blocking CAN interfaces via `embedded-can` traits
- Event-based notification using safe file descriptor operations
- Safe system call wrappers via the `nix` crate (no raw `extern "C"` calls)

## Requirements

- macOS (this library is macOS-specific)
- MacCAN PCBUSB library installed
- PEAK-System PCAN-USB compatible hardware

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pcbusb = "0.1"
embedded-can = "0.4"
nb = "1.1"
nix = "0.28"  # Used internally for safe POSIX system calls
```

### Basic Example

```rust
use pcbusb::prelude::*;
use pcbusb::{Filter, Frame, Interface, StandardId};
use embedded_can::blocking::Can as BlockingCan;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the CAN interface
    let mut can_interface = Interface::init()?;
    
    // Set up message filtering
    let filter = Filter::accept_all();
    can_interface.add_filter(&filter)?;
    
    // Create and send a test message
    let test_id = StandardId::new(0x123).unwrap();
    let test_data = [0x01, 0x02, 0x03, 0x04];
    let test_frame = Frame::new(test_id, &test_data).unwrap();
    
    BlockingCan::transmit(&mut can_interface, &test_frame)?;
    
    // Receive messages (blocking)
    let received_frame = BlockingCan::receive(&mut can_interface)?;
    println!("Received: {:?}", received_frame.data());
    
    Ok(())
}
```

See `examples/basic.rs` for a more complete example.

### Key Differences from Windows Version

This macOS version differs from typical Windows PCAN implementations in several ways:

- **Safe POSIX APIs**: Uses `std::os::unix` traits and the `nix` crate instead of raw system calls
- **Event Handling**: Uses safe `select()` with `BorrowedFd` instead of Windows events
- **Type Safety**: Custom `EventHandle` wrapper provides compile-time safety
- **No Manual Cleanup**: File descriptors are managed automatically by the MacCAN library
- **Native macOS Integration**: Built specifically for the MacCAN PCBUSB library

## Examples

Run the basic example with:

```bash
cargo run --example basic
```

This example demonstrates:
- Interface initialization
- Message transmission and reception
- Filter configuration
- Error handling

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

> **Note**: This library interfaces with MacCAN, which provides CAN support for macOS.
> Please ensure you have the appropriate rights to use your CAN hardware and comply
> with any vendor-specific licensing requirements for your PEAK-System devices.
>
> For PEAK-System hardware licensing information, please refer to:
> https://www.peak-system.com/quick/eula

# pcbusb-rs

Drives a PEAK-System PCAN-USB adapter through [`embedded-can`](https://crates.io/crates/embedded-can)'s
traits. On macOS it uses the MacCAN PCBUSB library, and elsewhere PEAK's PCAN-Basic, through
[`peak-can-sys`](https://crates.io/crates/peak-can-sys). It aims to mimic
[pcan-basic-rs](https://github.com/timokroeger/pcan-basic-rs).

The raw bindings to `PCBUSB.h` are committed in `pcbusb/src/sys/mac.rs`, so building needs neither
the header nor libclang. `pcbusb/src/sys.rs` has the command that regenerates them.

## Requirements

- On macOS, the [MacCAN PCBUSB library](https://www.mac-can.com) installed in `/usr/local/lib`
- Elsewhere, PEAK's PCAN-Basic driver
- A PCAN-USB adapter

## Usage

```rust
use embedded_can::blocking::Can;
use pcbusb::{Baudrate, Filter, Frame, Interface, StandardId};
use pcbusb::prelude::*;

fn main() -> Result<(), pcbusb::Error> {
    let mut can = Interface::init(Baudrate::Baud500k)?;
    can.add_filter(&Filter::accept_all())?;

    let frame = Frame::new(StandardId::new(0x123).unwrap(), &[1, 2, 3, 4]).unwrap();
    Can::transmit(&mut can, &frame)?;

    let received = Can::receive(&mut can)?;
    println!("Received {:?}", received.data());
    Ok(())
}
```

- `Interface::init_listen_only` opens the adapter without acknowledging or sending anything.
- `Interface::receive_timestamped` reads a frame with the adapter's hardware timestamp.
- `Baudrate` has PCAN-Basic's rates and 666.666 kbit/s, which it has no constant for.

`examples/basic.rs` is a fuller example: `cargo run --example basic`.

## License

MIT, see [LICENSE](LICENSE). PEAK-System's own licence covers its drivers and hardware:
https://www.peak-system.com/quick/eula

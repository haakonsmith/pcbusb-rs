# pcbusb-rs

Ergonomic wrapper for the MacCAN PCBUSB API for macOS. See `README.md` in the `pcbusb-sys` folder for more details on the raw bindings.

This aims to mimic https://github.com/timokroeger/pcan-basic-rs

## Features

- Safe Rust bindings for MacCAN PCBUSB library
- macOS-exclusive CAN interface support
- Compatible with PEAK-System PCAN-USB devices on macOS
- Type-safe API wrapper around the raw C bindings

## Requirements

- macOS (this library is macOS-specific)
- MacCAN PCBUSB library installed
- PEAK-System PCAN-USB compatible hardware

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

> **Note**: This library interfaces with MacCAN, which provides CAN support for macOS.
> Please ensure you have the appropriate rights to use your CAN hardware and comply
> with any vendor-specific licensing requirements for your PEAK-System devices.
>
> For PEAK-System hardware licensing information, please refer to:
> https://www.peak-system.com/quick/eula

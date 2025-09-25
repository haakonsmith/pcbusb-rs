# pcbusb-sys

Raw Rust bindings for the MacCAN PCBUSB library on macOS.

This crate provides unsafe, low-level bindings to the MacCAN PCBUSB C library, which enables CAN communication with PEAK-System PCAN-USB devices on macOS systems.

## Overview

`pcbusb-sys` contains the raw FFI bindings generated from the MacCAN PCBUSB headers. These bindings are automatically generated using `bindgen` and provide direct access to the underlying C API.

**Note**: These are raw, unsafe bindings. For a safe, ergonomic Rust API, use the parent `pcbusb` crate instead.

## Installation

The MacCAN PCBUSB library must be installed on your macOS system before using this crate. The build script will attempt to locate and link against the installed library.

## Building

This crate uses a build script (`build.rs`) that:
- Locates the MacCAN PCBUSB headers and library
- Generates Rust bindings using `bindgen`
- Links against the MacCAN PCBUSB library

This crate statically links against the pcbusb library (woo portability, bye bye binary sizes...).

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

> **Important**: This library interfaces with MacCAN and PEAK-System hardware.
> Please ensure you comply with all applicable licensing terms for the underlying
> MacCAN library and your PEAK-System hardware.
>
> For PEAK-System hardware licensing information, please refer to:
> https://www.peak-system.com/quick/eula

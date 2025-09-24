# macOS Conversion Notes

This document outlines the changes made to convert the Windows-specific pcbusb code to work natively on macOS using POSIX APIs and the MacCAN PCBUSB library.

## Overview

The original code was designed for Windows and used Windows-specific synchronization primitives. This conversion replaces those with POSIX equivalents while maintaining the same API surface and functionality.

## Key Changes Made

### 1. Removed Windows Dependencies

**Before:**
```rust
use winapi::{
    shared::minwindef::FALSE,
    um::{synchapi, winbase::INFINITE, winnt::HANDLE},
};
```

**After:**
```rust
#[cfg(unix)]
use nix::sys::select::{FdSet, select};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, BorrowedFd, RawFd};

/// A wrapper around a file descriptor provided by the MacCAN library
#[cfg(unix)]
#[derive(Debug)]
pub struct EventHandle {
    fd: RawFd,
}

#[cfg(unix)]
type HANDLE = EventHandle;
```

### 2. Replaced Windows Event Handling

**Before (Windows approach):**
```rust
let mut event_handle = unsafe { 
    synchapi::CreateEventA(ptr::null_mut(), FALSE, FALSE, ptr::null()) 
};
unsafe {
    CAN_SetValue(
        pcan_channel,
        PCAN_RECEIVE_EVENT as u8,
        &mut event_handle as *mut _ as *mut c_void,
        mem::size_of_val(&event_handle) as u32,
    );
};
```

**After (macOS approach):**
```rust
let mut event_handle: i32 = 0;
let result = unsafe {
    CAN_GetValue(  // Note: GET instead of SET
        pcan_channel,
        PCAN_RECEIVE_EVENT as u8,
        &mut event_handle as *mut _ as *mut c_void,
        mem::size_of_val(&event_handle) as u32,
    )
};
```

### 3. Added Safe POSIX System Calls via nix Crate

Instead of raw `extern "C"` declarations, we now use the `nix` crate for safe POSIX system call wrappers:

```rust
#[cfg(unix)]
use nix::sys::select::{FdSet, select};
```

Added to `Cargo.toml`:
```toml
nix = { version = "0.28", features = ["poll"] }
```

### 4. Implemented Type-Safe File Descriptor Management

```rust
/// A wrapper around a file descriptor provided by the MacCAN library
#[cfg(unix)]
#[derive(Debug)]
pub struct EventHandle {
    fd: RawFd,
}

#[cfg(unix)]
impl EventHandle {
    fn from_raw_fd(fd: RawFd) -> Self {
        Self { fd }
    }

    fn as_borrowed_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.fd) }
    }
}

#[cfg(unix)]
impl AsRawFd for EventHandle {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}
```

### 5. Replaced Windows Blocking Wait

**Before (Windows):**
```rust
unsafe { synchapi::WaitForSingleObject(self.event_handle, INFINITE) };
```

**After (macOS with safe Rust APIs):**
```rust
let mut readfds = FdSet::new();
readfds.insert(self.event_handle.as_borrowed_fd());

// Block indefinitely until the file descriptor is ready
select(None, Some(&mut readfds), None, None, None)?;
```

### 6. Enhanced Error Handling

Added proper implementation of `embedded_can::Error` and conversions from standard error types:
```rust
impl embedded_can::Error for Error {
    fn kind(&self) -> embedded_can::ErrorKind {
        embedded_can::ErrorKind::Other
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self(format!("I/O error: {}", err))
    }
}

impl From<nix::Error> for Error {
    fn from(err: nix::Error) -> Self {
        Self(format!("System error: {}", err))
    }
}
```

Corrected trait method signatures:
```rust
impl embedded_can::nb::Can for Interface {
    type Frame = Frame;
    type Error = Error;

    fn transmit(&mut self, frame: &Self::Frame) -> nb::Result<Option<Self::Frame>, Self::Error> {
        self.transmit_internal(frame)
    }

    fn receive(&mut self) -> nb::Result<Self::Frame, Self::Error> {
        self.receive_internal()
    }
}
```

### 7. Fixed Frame Implementation

Corrected return types in Frame creation methods:
```rust
// Before
return Err(());

// After
return None;
```

## Technical Details

### MacCAN Library Integration

The MacCAN library provides a file descriptor through `PCAN_RECEIVE_EVENT` that can be used with standard POSIX `select()` calls. This is different from Windows where you create an event object and pass it to the library.

**Key insight:** The MacCAN library documentation states:
> "Parameter PCAN_RECEIVE_EVENT returns a file descriptor to realize 'blocking read' by select() as on the Linux implementation of the PCAN-Basic API"

### Safe Rust Integration

We use standard Rust library features and the `nix` crate instead of raw `extern "C"` declarations:
- `std::os::unix::io::BorrowedFd` for safe file descriptor borrowing
- `std::os::unix::io::AsRawFd` trait for type-safe raw FD access
- `nix::sys::select` for safe POSIX select() calls
- Custom `EventHandle` wrapper for better type safety

### Memory Management

Unlike Windows handles that need explicit cleanup, file descriptors returned by MacCAN are managed by the library itself and don't require manual cleanup in the Drop implementation. Our `EventHandle` wrapper provides type safety without additional overhead.

### Error Handling

Enhanced error handling with automatic conversions from `std::io::Error` and `nix::Error` types, providing better integration with the Rust ecosystem while maintaining compatibility with PCAN error codes.

## Testing

A comprehensive example (`examples/basic.rs`) was created to demonstrate:
- Interface initialization
- Message transmission and reception  
- Filter configuration
- Both blocking and non-blocking API usage
- Proper error handling

## Compatibility

This implementation is macOS-specific and requires:
- MacCAN PCBUSB library installed
- PEAK-System PCAN-USB hardware
- macOS operating system

The API remains compatible with the original embedded-can traits, allowing existing code to work with minimal changes.

## Build Requirements

The code uses conditional compilation (`#[cfg(unix)]`) to ensure it only compiles on Unix-like systems, preventing accidental compilation on Windows where the POSIX APIs wouldn't be available.

### Dependencies

- `nix = { version = "0.28", features = ["poll"] }` - Safe POSIX system call wrappers
- `embedded-can = "0.4.1"` - CAN bus abstraction traits
- `nb = "1.1.0"` - Non-blocking I/O primitives

The `nix` crate provides safe, idiomatic Rust wrappers around POSIX system calls, eliminating the need for unsafe `extern "C"` declarations and manual memory management.
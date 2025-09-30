use crate::sys::{
    CAN_GetValue, CAN_Initialize, CAN_Read, CAN_SetValue, CAN_Uninitialize, CAN_Write,
    PCAN_ACCEPTANCE_FILTER_11BIT, PCAN_ACCEPTANCE_FILTER_29BIT, PCAN_ALLOW_STATUS_FRAMES,
    PCAN_ERROR_OK, PCAN_ERROR_QRCVEMPTY, PCAN_FILTER_CLOSE, PCAN_FILTER_CUSTOM, PCAN_FILTER_OPEN,
    PCAN_MESSAGE_FILTER, PCAN_PARAMETER_OFF, PCAN_RECEIVE_EVENT, PCAN_USBBUS1, TPCANMsg,
};
use crate::{Baudrate, Error, Filter, Frame};

use std::{
    ffi::c_void,
    mem::{self, MaybeUninit},
    ptr,
};

#[cfg(unix)]
use nix::sys::select::{FdSet, select};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, BorrowedFd, RawFd};

#[cfg(windows)]
use winapi::{
    shared::minwindef::FALSE,
    um::{synchapi, winbase::INFINITE, winnt},
};

/// A wrapper around a Windows HANDLE for event notification.
/// The HANDLE is managed by the PCAN library and doesn't require manual cleanup.
#[cfg(windows)]
#[derive(Debug)]
pub struct EventHandle {
    handle: winnt::HANDLE,
}

#[cfg(windows)]
unsafe impl Send for EventHandle {}

#[cfg(windows)]
impl EventHandle {
    /// Create a new EventHandle from a raw HANDLE
    fn from_handle(handle: winnt::HANDLE) -> Self {
        Self { handle }
    }

    /// Get the raw HANDLE
    fn as_handle(&self) -> winnt::HANDLE {
        self.handle
    }
}

/// A wrapper around a file descriptor provided by the MacCAN library
/// for event notification. The file descriptor is managed by the library
/// and doesn't require manual cleanup.
#[cfg(unix)]
#[derive(Debug)]
pub struct EventHandle {
    fd: RawFd,
}

#[cfg(unix)]
impl EventHandle {
    /// Create a new EventHandle from a raw file descriptor
    fn from_raw_fd(fd: RawFd) -> Self {
        Self { fd }
    }

    /// Get a borrowed file descriptor for use with system calls
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

#[cfg(unix)]
type HANDLE = EventHandle;

#[cfg(windows)]
type HANDLE = EventHandle;

pub struct Interface {
    channel: u16,
    event_handle: HANDLE,
    _baudrate: Baudrate,
}

impl Interface {
    pub fn init(baudrate: Baudrate) -> Result<Self, Error> {
        let pcan_channel = PCAN_USBBUS1 as u16;

        let result = unsafe { CAN_Initialize(pcan_channel, baudrate as u16, 0, 0, 0) };
        if result != PCAN_ERROR_OK {
            return Err(Error::new(result));
        }

        // let mut parameter_off = PCAN_PARAMETER_OFF;
        // unsafe {
        //     CAN_SetValue(
        //         pcan_channel,
        //         PCAN_ALLOW_STATUS_FRAMES as u8,
        //         &mut parameter_off as *mut _ as *mut c_void,
        //         mem::size_of_val(&parameter_off) as u32,
        //     );
        // }

        #[cfg(unix)]
        let mut raw_fd: RawFd = 0;

        #[cfg(windows)]
        let mut raw_fd =
            unsafe { synchapi::CreateEventA(ptr::null_mut(), FALSE, FALSE, ptr::null()) };
        #[cfg(windows)]
        if raw_fd.is_null() {
            return Err(Error::new(result));
        }

        #[cfg(windows)]
        unsafe {
            CAN_SetValue(
                pcan_channel,
                PCAN_RECEIVE_EVENT as u8,
                &mut raw_fd as *mut _ as *mut c_void,
                mem::size_of_val(&raw_fd) as u32,
            )
        };

        // #[cfg(unix)]
        // unsafe {
        //     CAN_GetValue(
        //         pcan_channel,
        //         PCAN_RECEIVE_EVENT as u8,
        //         &mut raw_fd as *mut _ as *mut c_void,
        //         mem::size_of_val(&raw_fd) as u32,
        //     )
        // };

        #[cfg(unix)]
        let event_handle = EventHandle::from_raw_fd(raw_fd);
        #[cfg(windows)]
        let event_handle = EventHandle::from_handle(raw_fd);

        let mut this = Self {
            channel: pcan_channel,
            event_handle,
            _baudrate: baudrate,
        };

        // Drain all messages that were received since `init()` has been called.
        loop {
            if let Err(nb::Error::WouldBlock) = this.receive_internal() {
                break;
            }
        }

        Ok(this)
    }
}

impl Drop for Interface {
    fn drop(&mut self) {
        unsafe {
            CAN_Uninitialize(self.channel);
        };
    }
}

impl Interface {
    fn transmit_internal(&mut self, frame: &Frame) -> nb::Result<Option<Frame>, Error> {
        let result = unsafe { CAN_Write(self.channel, &frame.0 as *const _ as *mut _) };
        if result == PCAN_ERROR_OK {
            Ok(None)
        } else {
            Err(nb::Error::Other(Error::new(result)))
        }
    }

    fn receive_internal(&mut self) -> nb::Result<Frame, Error> {
        let mut msg = MaybeUninit::<TPCANMsg>::uninit();
        let (result, msg) = unsafe {
            (
                CAN_Read(self.channel, msg.as_mut_ptr(), ptr::null_mut()),
                msg.assume_init(),
            )
        };

        match result {
            PCAN_ERROR_QRCVEMPTY => Err(nb::Error::WouldBlock),
            PCAN_ERROR_OK => Ok(Frame(msg)),
            _ => Err(nb::Error::Other(Error::new(result))),
        }
    }
}

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

impl embedded_can::blocking::Can for Interface {
    type Frame = Frame;
    type Error = Error;

    fn transmit(&mut self, frame: &Frame) -> Result<(), Error> {
        match self.transmit_internal(frame) {
            Ok(_) => Ok(()),
            Err(nb::Error::Other(err)) => Err(err),
            _ => panic!("The PCAN driver should never block!"),
        }
    }

    fn receive(&mut self) -> Result<Frame, Error> {
        loop {
            match self.receive_internal() {
                Ok(frame) => break Ok(frame),
                Err(nb::Error::Other(err)) => break Err(err),
                Err(nb::Error::WouldBlock) => continue,
                _ => panic!("Receive queue should not be empty!"),
            }
        }
    }
}

impl Interface {
    pub fn add_filter(&mut self, filter: &Filter) -> Result<(), Error> {
        let mut filter_state = 0u32;
        unsafe {
            CAN_GetValue(
                self.channel,
                PCAN_MESSAGE_FILTER as u8,
                &mut filter_state as *mut _ as *mut c_void,
                mem::size_of_val(&filter_state) as u32,
            );
        }
        if filter_state == PCAN_FILTER_CUSTOM {
            return Err(Error("Cannot configure more than one filter".to_string()));
        }

        if filter.accept_all {
            let mut filter_open = PCAN_FILTER_OPEN;
            unsafe {
                CAN_SetValue(
                    self.channel,
                    PCAN_MESSAGE_FILTER as u8,
                    &mut filter_open as *mut _ as *mut c_void,
                    mem::size_of_val(&filter_open) as u32,
                );
            };
        } else {
            let mut value = [filter.mask.to_le(), filter.id.to_le()];
            unsafe {
                CAN_SetValue(
                    self.channel,
                    if filter.is_extended {
                        PCAN_ACCEPTANCE_FILTER_29BIT
                    } else {
                        PCAN_ACCEPTANCE_FILTER_11BIT
                    } as u8,
                    &mut value as *mut _ as *mut c_void,
                    mem::size_of_val(&value) as u32,
                );
            };
        }

        Ok(())
    }

    pub fn clear_filters(&mut self) {
        let mut filter_open = PCAN_FILTER_CLOSE;
        unsafe {
            CAN_SetValue(
                self.channel,
                PCAN_MESSAGE_FILTER as u8,
                &mut filter_open as *mut _ as *mut c_void,
                mem::size_of_val(&filter_open) as u32,
            );
        };
    }
}

#[cfg(target_os = "macos")]
pub use mac_can_sys::*;

#[cfg(not(target_os = "macos"))]
mod peak_compat {
    pub use peak_can_sys::*;
    use std::ffi::c_char;

    // Platform-specific DWORD handling
    #[cfg(target_os = "windows")]
    type PlatformDWORD = u32;
    #[cfg(not(target_os = "windows"))]
    type PlatformDWORD = u64;

    // Type aliases to match macOS API expectations
    pub type TPCANStatus = u32;

    // Re-export the message type with the expected name
    pub use peak_can_sys::CANTPMsg as TPCANMsg;

    // Constants - convert from u32 to match expected types
    pub const PCAN_ERROR_OK: u32 = peak_can_sys::PEAK_ERROR_OK;
    pub const PCAN_ERROR_QRCVEMPTY: u32 = peak_can_sys::PEAK_ERROR_QRCVEMPTY;

    pub const PCAN_BAUD_1M: u32 = peak_can_sys::PEAK_BAUD_1M as u32;
    pub const PCAN_BAUD_5K: u32 = peak_can_sys::PEAK_BAUD_5K as u32;
    pub const PCAN_BAUD_10K: u32 = peak_can_sys::PEAK_BAUD_10K as u32;
    pub const PCAN_BAUD_20K: u32 = peak_can_sys::PEAK_BAUD_20K as u32;
    pub const PCAN_BAUD_33K: u32 = peak_can_sys::PEAK_BAUD_33K as u32;
    pub const PCAN_BAUD_47K: u32 = peak_can_sys::PEAK_BAUD_47K as u32;
    pub const PCAN_BAUD_50K: u32 = peak_can_sys::PEAK_BAUD_50K as u32;
    pub const PCAN_BAUD_83K: u32 = peak_can_sys::PEAK_BAUD_83K as u32;
    pub const PCAN_BAUD_95K: u32 = peak_can_sys::PEAK_BAUD_95K as u32;
    pub const PCAN_BAUD_100K: u32 = peak_can_sys::PEAK_BAUD_100K as u32;
    pub const PCAN_BAUD_125K: u32 = peak_can_sys::PEAK_BAUD_125K as u32;
    pub const PCAN_BAUD_250K: u32 = peak_can_sys::PEAK_BAUD_250K as u32;
    pub const PCAN_BAUD_500K: u32 = peak_can_sys::PEAK_BAUD_500K as u32;
    pub const PCAN_BAUD_800K: u32 = peak_can_sys::PEAK_BAUD_800K as u32;

    pub const PCAN_ACCEPTANCE_FILTER_11BIT: u32 = peak_can_sys::PEAK_ACCEPTANCE_FILTER_11BIT as u32;
    pub const PCAN_ACCEPTANCE_FILTER_29BIT: u32 = peak_can_sys::PEAK_ACCEPTANCE_FILTER_29BIT as u32;
    pub const PCAN_ALLOW_STATUS_FRAMES: u32 = peak_can_sys::PEAK_ALLOW_STATUS_FRAMES as u32;

    pub const PCAN_FILTER_CLOSE: u32 = peak_can_sys::PEAK_FILTER_CLOSE as u32;
    pub const PCAN_FILTER_CUSTOM: u32 = peak_can_sys::PEAK_FILTER_CUSTOM as u32;
    pub const PCAN_FILTER_OPEN: u32 = peak_can_sys::PEAK_FILTER_OPEN as u32;

    pub const PCAN_MESSAGE_EXTENDED: u8 = peak_can_sys::PEAK_MESSAGE_EXTENDED as u8;
    pub const PCAN_MESSAGE_FILTER: u32 = peak_can_sys::PEAK_MESSAGE_FILTER as u32;
    pub const PCAN_MESSAGE_RTR: u8 = peak_can_sys::PEAK_MESSAGE_RTR as u8;
    pub const PCAN_MESSAGE_STANDARD: u8 = peak_can_sys::PEAK_MESSAGE_STANDARD as u8;

    pub const PCAN_PARAMETER_OFF: u32 = peak_can_sys::PEAK_PARAMETER_OFF as u32;
    pub const PCAN_RECEIVE_EVENT: u32 = peak_can_sys::PEAK_RECEIVE_EVENT as u32;
    pub const PCAN_USBBUS1: u16 = peak_can_sys::PEAK_USBBUS1 as u16;

    // Wrapper functions to handle type conversions
    pub unsafe fn CAN_GetErrorText(error: u32, language: u16, buffer: *mut c_char) -> u32 {
        let result =
            unsafe { peak_can_sys::CAN_GetErrorText(error as PlatformDWORD, language, buffer) };
        result as u32
    }

    pub unsafe fn CAN_Initialize(
        channel: u16,
        btr0btr1: u16,
        hw_type: u8,
        io_port: u32,
        interrupt: u16,
    ) -> u32 {
        let result = unsafe {
            peak_can_sys::CAN_Initialize(
                channel,
                btr0btr1,
                hw_type,
                io_port as PlatformDWORD,
                interrupt,
            )
        };
        result as u32
    }

    pub unsafe fn CAN_Uninitialize(channel: u16) -> u32 {
        let result = unsafe { peak_can_sys::CAN_Uninitialize(channel) };
        result as u32
    }

    pub unsafe fn CAN_Reset(channel: u16) -> u32 {
        let result = unsafe { peak_can_sys::CAN_Reset(channel) };
        result as u32
    }

    pub unsafe fn CAN_GetStatus(channel: u16) -> u32 {
        let result = unsafe { peak_can_sys::CAN_GetStatus(channel) };
        result as u32
    }

    pub unsafe fn CAN_Read(
        channel: u16,
        msg: *mut TPCANMsg,
        timestamp: *mut peak_can_sys::CANTPTimestamp,
    ) -> u32 {
        let result = unsafe { peak_can_sys::CAN_Read(channel, msg, timestamp) };
        result as u32
    }

    pub unsafe fn CAN_Write(channel: u16, msg: *const TPCANMsg) -> u32 {
        let result = unsafe { peak_can_sys::CAN_Write(channel, msg as *mut TPCANMsg) };
        result as u32
    }

    pub unsafe fn CAN_FilterMessages(channel: u16, from_id: u32, to_id: u32, mode: u8) -> u32 {
        let result = unsafe {
            peak_can_sys::CAN_FilterMessages(
                channel,
                from_id as PlatformDWORD,
                to_id as PlatformDWORD,
                mode,
            )
        };
        result as u32
    }

    pub unsafe fn CAN_GetValue(
        channel: u16,
        parameter: u8,
        buffer: *mut std::ffi::c_void,
        buffer_length: u32,
    ) -> u32 {
        let result = unsafe {
            peak_can_sys::CAN_GetValue(channel, parameter, buffer, buffer_length as PlatformDWORD)
        };
        result as u32
    }

    pub unsafe fn CAN_SetValue(
        channel: u16,
        parameter: u8,
        buffer: *const std::ffi::c_void,
        buffer_length: u32,
    ) -> u32 {
        let result = unsafe {
            peak_can_sys::CAN_SetValue(
                channel,
                parameter,
                buffer as *mut std::ffi::c_void,
                buffer_length as PlatformDWORD,
            )
        };
        result as u32
    }

    // Custom message conversion helpers for handling ID field size differences
    pub fn convert_msg_for_reading(peak_msg: &peak_can_sys::CANTPMsg) -> TPCANMsg {
        // Since TPCANMsg is just an alias for CANTPMsg, we can return it directly
        // but we need to be careful about the ID field size in usage
        *peak_msg
    }

    pub fn convert_msg_for_writing(msg: &TPCANMsg) -> peak_can_sys::CANTPMsg {
        // Direct conversion since they're the same type
        *msg
    }
}

#[cfg(not(target_os = "macos"))]
pub use peak_compat::*;

use embedded_can::ExtendedId;
use embedded_can::Id;
use embedded_can::StandardId;

use crate::sys::PCAN_MESSAGE_EXTENDED;
use crate::sys::PCAN_MESSAGE_RTR;
use crate::sys::PCAN_MESSAGE_STANDARD;
use crate::sys::TPCANMsg;

#[derive(Debug)]
pub struct Frame(pub(crate) TPCANMsg);

impl embedded_can::Frame for Frame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Frame> {
        if data.len() > 8 {
            return None;
        }

        let (id, msg_type) = match id.into() {
            Id::Standard(id) => (id.as_raw() as u32, PCAN_MESSAGE_STANDARD),
            Id::Extended(id) => (id.as_raw(), PCAN_MESSAGE_EXTENDED),
        };

        let mut msg = TPCANMsg {
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            ID: id,
            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
            ID: id as u64,
            MSGTYPE: msg_type as u8,
            LEN: data.len() as u8,
            DATA: [0; 8],
        };
        msg.DATA[0..data.len()].copy_from_slice(data);
        Some(Frame(msg))
    }

    fn new_remote(id: impl Into<Id>, dlc: usize) -> Option<Frame> {
        if dlc >= 8 {
            return None;
        }

        let mut frame = Frame::new(id, &[])?;
        frame.0.MSGTYPE |= PCAN_MESSAGE_RTR as u8;
        frame.0.LEN = dlc as u8;
        Some(frame)
    }

    fn is_extended(&self) -> bool {
        self.0.MSGTYPE & PCAN_MESSAGE_EXTENDED as u8 != 0
    }

    fn is_remote_frame(&self) -> bool {
        self.0.MSGTYPE & PCAN_MESSAGE_RTR as u8 != 0
    }

    fn id(&self) -> Id {
        if self.is_extended() {
            ExtendedId::new(self.0.ID as u32).unwrap().into()
        } else {
            StandardId::new(self.0.ID as u16).unwrap().into()
        }
    }

    fn dlc(&self) -> usize {
        self.0.LEN as usize
    }

    fn data(&self) -> &[u8] {
        &self.0.DATA[0..self.0.LEN as usize]
    }
}

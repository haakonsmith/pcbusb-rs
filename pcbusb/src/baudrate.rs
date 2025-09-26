use crate::sys::*;

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum Baudrate {
    Baud1m = PCAN_BAUD_1M as u16,
    Baud800k = PCAN_BAUD_800K as u16,
    Baud500k = PCAN_BAUD_500K as u16,
    Baud250k = PCAN_BAUD_250K as u16,
    Baud125k = PCAN_BAUD_125K as u16,
    Baud100k = PCAN_BAUD_100K as u16,
    Baud95k = PCAN_BAUD_95K as u16,
    Baud83k = PCAN_BAUD_83K as u16,
    Baud50k = PCAN_BAUD_50K as u16,
    Baud47k = PCAN_BAUD_47K as u16,
    Baud33k = PCAN_BAUD_33K as u16,
    Baud20k = PCAN_BAUD_20K as u16,
    Baud10k = PCAN_BAUD_10K as u16,
    Baud5k = PCAN_BAUD_5K as u16,
}

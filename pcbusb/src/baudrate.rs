use crate::sys::*;

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum Baudrate {
    Baud1m = PCAN_BAUD_1M as u16,
    Baud800k = PCAN_BAUD_800K as u16,
    /// 666,666.67 bit/s, which PCAN-Basic has no constant for: the BTR0/BTR1 code of 12 time
    /// quanta of 125 ns (the 8 MHz clock its codes are for, at prescaler 1), sampled at 10 of them,
    /// 83.3%, with a jump width of 1. Some transmission control units' plant mode runs at it
    Baud666k = 0x0018,
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

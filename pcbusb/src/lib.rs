pub mod prelude {
    pub use embedded_can::{Frame as _, nb, nb::Can as _};
}

pub use embedded_can::{ExtendedId, Id, StandardId};

mod baudrate;
mod error;
mod filter;
mod frame;
mod interface;
mod sys;

pub use baudrate::Baudrate;
pub use error::Error;
pub use filter::Filter;
pub use frame::Frame;
pub use interface::Interface;

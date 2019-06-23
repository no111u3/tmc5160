//! A platform agnostic driver to iterface with the TMC5160 (Trinamic integrated stepper motor controller)
//!
//! This driver wa built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/0.2
//!
//#![deny(missing_docs)]
#![no_std]

use embedded_hal::{
    blocking::spi::{Transfer, Write},
    digital::v2::OutputPin,
    spi::{Mode, Phase, Polarity}
};

/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleLow,
};

/// TMC5160 driver
pub struct Tmc5160<SPI, CS> {
    spi: SPI,
    cs: CS,
}

impl<SPI, CS, E> Tle5012<SPI, CS>
where
    SPI: Transfer<u8, Error = E> + Write<u8, Error = E>,
    CS: OutputPin,
{
    /// Create a new driver from a SPI peripheral and a NCS pin
    pub fn new(spi: SPI, cs: CS) -> Result<Self, E> {
        Ok(Tmc5160 { spi, cs })
    }
}

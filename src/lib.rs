//! A platform agnostic driver to iterface with the TMC5160 (Trinamic integrated stepper motor controller)
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/0.2
//!
#![no_std]
#![allow(dead_code)]
//#![deny(missing_docs)]
#![deny(warnings)]

use core::fmt;
use core::result::Result;

use embedded_hal::{
    blocking::spi::{Transfer, Write},
    digital::v2::OutputPin,
    spi::{Mode, Phase, Polarity},
};

use crate::registers::Address;

mod registers;

/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleHigh,
};

/// TMC5160 driver
pub struct Tmc5160<SPI, CS> {
    spi: SPI,
    cs: CS,
}

#[derive(Debug)]
pub enum Error<E> {
    /// SPI bus error
    Spi(E),
}

/// Data Exchange packet
#[derive(Debug)]
pub struct DataPacket(u8, u32);

impl fmt::Display for DataPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x}:0x{:x}", self.0, self.1)
    }
}

impl<SPI, CS, E> Tmc5160<SPI, CS>
    where
        SPI: Transfer<u8, Error=E> + Write<u8, Error=E>,
        CS: OutputPin,
{
    /// Create a new driver from a SPI peripheral and a NCS pin
    pub fn new(spi: SPI, cs: CS) -> Result<Self, E> {
        Ok(Tmc5160 { spi, cs })
    }

    pub fn read_register<T>(&mut self, reg: T) -> Result<DataPacket, Error<E>>
        where
            T: Address + Copy,
    {
        // Process cmd to read, return previous(dummy) state
        let _dummy = self.read_io(reg)?;
        // Repeat cmd to read, return state
        self.read_io(reg)
    }

    fn read_io<T>(&mut self, reg: T) -> Result<DataPacket, Error<E>>
        where
            T: Address + Copy,
    {
        self.cs.set_low().ok();

        let mut buffer = [reg.addr() & 0x7f];

        self.spi.transfer(&mut buffer).map_err(Error::Spi)?;

        let mut ret_val: [u8; 4] = [0; 4];

        self.spi.transfer(&mut ret_val).map_err(Error::Spi)?;

        self.cs.set_high().ok();

        Ok(DataPacket(buffer[0], u32::from_be_bytes(ret_val)))
    }

    pub fn write_register<T>(&mut self, reg: T, data: u32) -> Result<DataPacket, Error<E>>
        where
            T: Address + Copy,
    {
        self.cs.set_low().ok();

        let mut buffer = [reg.addr() | 0x80];

        self.spi.transfer(&mut buffer).map_err(Error::Spi)?;

        let mut val = data.to_be_bytes();

        self.spi.transfer(&mut val).map_err(Error::Spi)?;

        self.cs.set_high().ok();

        Ok(DataPacket(buffer[0], u32::from_be_bytes(val)))
    }
}

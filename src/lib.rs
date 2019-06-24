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


#[derive(Debug)]
pub enum Error<E> {
    /// SPI bus error
    Spi(E)
}

pub trait Address {
    fn addr(self) -> u8;
}

impl<SPI, CS, E> Tmc5160<SPI, CS>
where
    SPI: Transfer<u8, Error = E> + Write<u8, Error = E>,
    CS: OutputPin,
{
    /// Create a new driver from a SPI peripheral and a NCS pin
    pub fn new(spi: SPI, cs: CS) -> Result<Self, E> {
        Ok(Tmc5160 { spi, cs })
    }

    pub fn read_register<T>(&mut self, reg: T) -> Result<u32, Error<E>>
    where
        T: Address
    {
        self.cs.set_low().ok();
        
        let mut buffer = [reg.addr() & 0x7f];
        
        self.spi.transfer(&mut buffer).map_err(Error::Spi)?;

        let mut ret_val:[u8; 4] = [0; 4];
        
        self.spi.transfer(&mut ret_val).map_err(Error::Spi)?;
        
        self.cs.set_high().ok();
        
        Ok(u32::from_be_bytes(ret_val))
    }

    pub fn write_register<T>(&mut self, reg: T, data: u32) -> Result<u8, Error<E>>
    where
        T: Address
    {
        self.cs.set_low().ok();

        let mut buffer = [reg.addr() | 0x80];
        
        self.spi.transfer(&mut buffer).map_err(Error::Spi)?;
        
        let mut val = data.to_be_bytes();

        self.spi.transfer(&mut val).map_err(Error::Spi)?;
        
        self.cs.set_high().ok();

        Ok(buffer[0])
    }
}

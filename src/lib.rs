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
    Spi(E)
}

pub trait Address {
    fn addr(self) -> u8;
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Registers {
    GCONF = 0x00,
    GSTAT = 0x01,
    IFCNT = 0x02,
    SLAVECONF = 0x03,
    INP_OUT = 0x04,
    X_COMPARE = 0x05,
    OTP_PROG = 0x06,
    OTP_READ = 0x07,
    FACTORY_CONF = 0x08,
    SHORT_CONF = 0x09,
    DRV_CONF = 0x0A,
    GLOBAL_SCALER = 0x0B,
    OFFSET_READ = 0x0C,
    IHOLD_IRUN = 0x10,
    TPOWERDOWN = 0x11,
    TSTEP = 0x12,
    TPWMTHRS = 0x13,
    TCOOLTHRS = 0x14,
    THIGH = 0x15,

    RAMPMODE = 0x20,
    XACTUAL = 0x21,
    VACTUAL = 0x22,
    VSTART = 0x23,
    A1 = 0x24,
    V1 = 0x25,
    AMAX = 0x26,
    VMAX = 0x27,
    DMAX = 0x28,
    D1 = 0x2A,
    VSTOP = 0x2B,
    TZEROWAIT = 0x2C,
    XTARGET = 0x2D,

    VDCMIN = 0x33,
    SWMODE = 0x34,
    RAMPSTAT = 0x35,
    XLATCH = 0x36,
    ENCMODE = 0x38,
    XENC = 0x39,
    ENC_CONST = 0x3A,
    ENC_STATUS = 0x3B,
    ENC_LATCH = 0x3C,
    ENC_DEVIATION = 0x3D,

    MSLUT0 = 0x60,
    MSLUT1 = 0x61,
    MSLUT2 = 0x62,
    MSLUT3 = 0x63,
    MSLUT4 = 0x64,
    MSLUT5 = 0x65,
    MSLUT6 = 0x66,
    MSLUT7 = 0x67,
    MSLUTSEL = 0x68,
    MSLUTSTART = 0x69,
    MSCNT = 0x6A,
    MSCURACT = 0x6B,
    CHOPCONF = 0x6C,
    COOLCONF = 0x6D,
    DCCTRL = 0x6E,
    DRVSTATUS = 0x6F,
    PWMCONF = 0x70,
    PWMSCALE = 0x71,
    PWM_AUTO = 0x72,
    LOST_STEPS = 0x73
}

impl Address for Registers {
    fn addr(self) -> u8 {
        self as u8
    }
}

/// Data Exchange packet
#[derive(Debug)]
pub struct DataPacket(u8, u32);

use core::fmt;

impl fmt::Display for DataPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x}:0x{:x}", self.0, self.1)
    }
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

    pub fn read_register<T>(&mut self, reg: T) -> Result<DataPacket, Error<E>>
    where
        T: Address
    {
        self.cs.set_low().ok();
        
        let mut buffer = [reg.addr() & 0x7f];
        
        self.spi.transfer(&mut buffer).map_err(Error::Spi)?;

        let mut ret_val:[u8; 4] = [0; 4];
        
        self.spi.transfer(&mut ret_val).map_err(Error::Spi)?;
        
        self.cs.set_high().ok();
        
        Ok(DataPacket(buffer[0], u32::from_be_bytes(ret_val)))
    }

    pub fn write_register<T>(&mut self, reg: T, data: u32) -> Result<DataPacket, Error<E>>
    where
        T: Address
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

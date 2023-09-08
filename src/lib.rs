//! A platform agnostic driver to iterface with the TMC5160 (Trinamic integrated stepper motor controller)
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/0.2
//!
#![no_std]
#![allow(dead_code)]
#![deny(missing_docs)]
#![deny(warnings)]

use core::fmt;
use core::result::Result;

use embedded_hal::{
    blocking::spi::{Transfer, Write},
    digital::v2::OutputPin,
    spi::{Mode, Phase, Polarity},
};

use crate::registers::{Address, Registers};

mod registers;

/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleHigh,
};

/// TMC5160 driver
pub struct Tmc5160<SPI, CS, EN> {
    spi: SPI,
    cs: CS,
    en: Option<EN>,
    /// the max velocity that is set
    pub v_max: f32,
    _clock: f32,
    _step_count: f32,
}

#[derive(Debug)]
/// Error type for the TMC5160
pub enum Error<E> {
    /// SPI bus error
    Spi(E),
    /// Pin error
    PinError(E),
}

/// Data Exchange packet
#[derive(Debug)]
pub struct DataPacket(u8, u32);

impl fmt::Display for DataPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x}:0x{:x}", self.0, self.1)
    }
}

impl<SPI, CS, EN, E> Tmc5160<SPI, CS, EN>
    where
        SPI: Transfer<u8, Error=E> + Write<u8, Error=E>,
        CS: OutputPin<Error=E>,
        EN: OutputPin<Error=E>,
{
    /// Create a new driver from a SPI peripheral and a NCS pin
    pub fn new(spi: SPI, cs: CS) -> Self {
        Tmc5160 {
            spi,
            cs,
            en: None,
            v_max: 0.0,
            _clock: 12000000.0,
            _step_count: 256.0,
        }
    }

    /// add an enable pin to the driver
    pub fn en(mut self, en: EN) -> Self {
        self.en = Some(en);
        self
    }

    /// specify clock speed of the Tmc5160
    pub fn clock(mut self, clock: f32) -> Self {
        self._clock = clock;
        self
    }

    /// specify step count of the motor
    pub fn step_count(mut self, step_count: f32) -> Self {
        self._step_count = step_count;
        self
    }

    /// read a specified register
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

    /// write value to a specified register
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


    fn speed_from_hz(&mut self, speed_hz: f32) -> u32 {
        return (speed_hz / (self._clock / 16_777_216.0) * self._step_count) as u32;
    }

    fn accel_from_hz(&mut self, accel_hz_per_s: f32) -> u32 {
        return (accel_hz_per_s / (self._clock * self._clock)
            * (512.0 * 256.0)
            * 16_777_216.0
            * self._step_count) as u32;
    }

    /// enable the motor if the EN pin was specified
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        if let Some(pin) = &mut self.en {
            pin.set_low().map_err(Error::PinError)
        } else {
            Ok(())
        }
    }

    /// disable the motor if the EN pin was specified
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        if let Some(pin) = &mut self.en {
            pin.set_high().map_err(Error::PinError)
        } else {
            Ok(())
        }
    }

    /// set the position to 0 / home
    pub fn set_home(&mut self) -> Result<DataPacket, Error<E>> {
        self.write_register(Registers::XACTUAL, 0)?;
        self.write_register(Registers::XTARGET, 0)
    }

    /// stop the motor now
    pub fn stop(&mut self) -> Result<DataPacket, Error<E>> {
        self.disable()?;
        self.write_register(Registers::VSTART, 0)?;
        self.write_register(Registers::VMAX, 0)
    }

    /// check if the motor is moving
    pub fn is_moving(&mut self) -> Result<bool, Error<E>> {
        self.get_drv_status().map(|packet| (packet.0 & 0b1000) != 0b1000)
    }

    /// get the value of the DRV STATUS register
    pub fn get_drv_status(&mut self) -> Result<DataPacket, Error<E>> {
        self.read_register(Registers::DRV_STATUS)
    }

    /// set the max velocity (VMAX)
    pub fn set_velocity(&mut self, velocity: f32) -> Result<DataPacket, Error<E>> {
        self.v_max = velocity;
        let v_max = self.speed_from_hz(velocity);
        self.write_register(Registers::VMAX, v_max)
    }

    /// set the max acceleration (AMAX, DMAX, A1, D1)
    pub fn set_acceleration(&mut self, acceleration: f32) -> Result<DataPacket, Error<E>> {
        let a_max = self.accel_from_hz(acceleration);
        self.write_register(Registers::AMAX, a_max)?;
        self.write_register(Registers::DMAX, a_max)?;
        self.write_register(Registers::A_1, a_max)?;
        self.write_register(Registers::D_1, a_max)
    }

    /// move to a specific location
    pub fn move_to(&mut self, target_signed: i32) -> Result<DataPacket, Error<E>> {
        self.enable()?;
        let target = (target_signed * self._step_count as i32) as u32;
        self.write_register(Registers::XTARGET, target)
    }

    /// get the current position
    pub fn get_position(&mut self) -> Result<f32, Error<E>> {
        self.read_register(Registers::XACTUAL).map(|val| val.1 as f32 / self._step_count)
    }

    /// set the current position
    pub fn set_position(&mut self, target_signed: i32) -> Result<DataPacket, Error<E>> {
        let target = target_signed as u32;
        self.write_register(Registers::XACTUAL, target * self._step_count as u32)
    }

    /// get the current velocity
    pub fn get_velocity(&mut self) -> Result<f32, Error<E>> {
        self.read_register(Registers::VACTUAL).map(|target| {
            if (target.1 & 0b100000000000000000000000) == 0b100000000000000000000000 {
                ((16777216 - target.1 as i32) as f64 / self._step_count as f64) as f32
            } else {
                ((target.1 as i32) as f64 / self._step_count as f64) as f32
            }
        })
    }

    /// get the set maximum velocity (VMAX)
    pub fn get_velocity_max(&mut self) -> f32 {
        self.v_max / 400.0
    }

    /// get the current target position (XTARGET)
    pub fn get_target(&mut self) -> Result<i32, Error<E>> {
        self.read_register(Registers::XTARGET).map(|packet| packet.1 as i32)
    }
}

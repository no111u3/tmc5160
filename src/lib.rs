//! A platform agnostic driver to interface with the TMC5160 (Trinamic integrated stepper motor controller)
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

use crate::registers::*;

pub mod registers;

/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleHigh,
};

#[derive(Debug)]
/// Error type for the TMC5160
pub enum Error<E> {
    /// SPI bus error
    Spi(E),
    /// Pin error
    PinError,
}

/// Data Exchange packet
pub struct DataPacket {
    /// Status returned from last communication
    pub status: SpiStatus,
    /// Data received from TMC5160
    pub data: u32,
}

impl fmt::Display for DataPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x}:0x{:x}", self.status.into_bytes()[0], self.data)
    }
}

/// TMC5160 driver
pub struct Tmc5160<SPI, CS, EN> {
    spi: SPI,
    cs: CS,
    en: Option<EN>,
    /// the max velocity that is set
    pub v_max: f32,
    /// status register of the driver
    pub status: SpiStatus,
    _clock: f32,
    _step_count: f32,
    _en_inverted: bool,
    /// value of the GCONF register
    pub g_conf: GConf,
    /// value of the NODECONF register
    pub node_conf: NodeConf,
    /// value of the OTPPROG register
    pub otp_prog: OtpProg,
    /// value of the SHORT_CONF register
    pub short_conf: ShortConf,
    /// value of the DRV_CONF register
    pub drv_conf: DrvConf,
    /// value of the IHOLD_IRUN register
    pub ihold_irun: IHoldIRun,
    /// value of the SWMODE register
    pub sw_mode: SwMode,
    /// value of the ENCMODE register
    pub enc_mode: EncMode,
    /// value of the MSLUTSEL register
    pub ms_lut_sel: MsLutSel,
    /// value of the CHOPCONF register
    pub chop_conf: ChopConf,
    /// value of the COOLCONF register
    pub cool_conf: CoolConf,
    /// value of the PWMCONF register
    pub pwm_conf: PwmConf,
}

impl<SPI, CS, EN, E> Tmc5160<SPI, CS, EN>
    where
        SPI: Transfer<u8, Error=E> + Write<u8, Error=E>,
        CS: OutputPin,
        EN: OutputPin,
{
    /// Create a new driver from a SPI peripheral and a NCS pin
    pub fn new(spi: SPI, cs: CS) -> Self {
        Tmc5160 {
            spi,
            cs,
            en: None,
            v_max: 0.0,
            status: SpiStatus::new(),
            _clock: 12000000.0,
            _step_count: 256.0,
            _en_inverted: false,
            g_conf: GConf::new(),
            node_conf: NodeConf::new(),
            otp_prog: OtpProg::new(),
            short_conf: ShortConf::new(),
            drv_conf: DrvConf::new(),
            ihold_irun: IHoldIRun::new(),
            sw_mode: SwMode::new(),
            enc_mode: EncMode::new(),
            ms_lut_sel: MsLutSel::new(),
            chop_conf: ChopConf::new(),
            cool_conf: CoolConf::new(),
            pwm_conf: PwmConf::new(),
        }
    }

    /// add an enable pin to the driver
    pub fn en(mut self, en: EN) -> Self {
        self.en = Some(en);
        self
    }

    /// invert the enable pin
    pub fn en_inverted(mut self, inv: bool) -> Self {
        self._en_inverted = inv;
        self
    }

    /// specify clock speed of the Tmc5160 (Default is 12 MHz)
    pub fn clock(mut self, clock: f32) -> Self {
        self._clock = clock;
        self
    }

    /// specify step count of the motor (Default is 256)
    pub fn step_count(mut self, step_count: f32) -> Self {
        self._step_count = step_count;
        self
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

    /// read a specified register
    pub fn read_register<T>(&mut self, reg: T) -> Result<DataPacket, Error<E>>
        where
            T: Address + Copy,
    {
        // Process cmd to read, return previous (dummy) state
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

        Ok(DataPacket { status: SpiStatus::from_bytes(buffer), data: u32::from_be_bytes(ret_val) })
    }

    /// write value to a specified register
    pub fn write_register<T>(&mut self, reg: T, val: &mut [u8; 4]) -> Result<DataPacket, Error<E>>
        where
            T: Address + Copy,
    {
        self.cs.set_low().ok();

        let mut buffer = [reg.addr() | 0x80];

        self.spi.transfer(&mut buffer).map_err(Error::Spi)?;

        //let mut val = data.to_be_bytes();

        self.spi.transfer(val).map_err(Error::Spi)?;

        self.cs.set_high().ok();

        Ok(DataPacket { status: SpiStatus::from_bytes([buffer[0]]), data: u32::from_be_bytes(*val) })
    }

    /// enable the motor if the EN pin was specified
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        if let Some(pin) = &mut self.en {
            if self._en_inverted {
                pin.set_high().map_err(|_| Error::PinError)
            } else {
                pin.set_low().map_err(|_| Error::PinError)
            }
        } else {
            Ok(())
        }
    }

    /// disable the motor if the EN pin was specified
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        if let Some(pin) = &mut self.en {
            if self._en_inverted {
                pin.set_low().map_err(|_| Error::PinError)
            } else {
                pin.set_high().map_err(|_| Error::PinError)
            }
        } else {
            Ok(())
        }
    }

    /// clear G_STAT register
    pub fn clear_g_stat(&mut self) -> Result<DataPacket, Error<E>> {
        let mut value = 0b111_u32.to_be_bytes();
        self.write_register(Registers::GCONF, &mut value)
    }

    /// write value to SW_MODE register
    pub fn update_sw_mode(&mut self) -> Result<DataPacket, Error<E>> {
        let mut value = self.sw_mode.into_bytes();
        self.write_register(Registers::SW_MODE, &mut value)
    }

    /// write value to G_CONF register
    pub fn update_g_conf(&mut self) -> Result<DataPacket, Error<E>> {
        let mut value = self.g_conf.into_bytes();
        self.write_register(Registers::GCONF, &mut value)
    }

    /// write value to CHOP_CONF register
    pub fn update_chop_conf(&mut self) -> Result<DataPacket, Error<E>> {
        let mut value = self.chop_conf.into_bytes();
        self.write_register(Registers::CHOPCONF, &mut value)
    }

    /// write value to IHOLD_IRUN register
    pub fn update_ihold_irun(&mut self) -> Result<DataPacket, Error<E>> {
        let mut value = self.ihold_irun.into_bytes();
        self.write_register(Registers::IHOLD_IRUN, &mut value)
    }

    /// write value to PWM_CONF register
    pub fn update_pwm_conf(&mut self) -> Result<DataPacket, Error<E>> {
        let mut value = self.pwm_conf.into_bytes();
        self.write_register(Registers::PWMCONF, &mut value)
    }

    /// write value to GLOBALSCALER register
    pub fn set_global_scaler(&mut self, val: u32) -> Result<DataPacket, Error<E>> {
        let mut value = val.to_be_bytes();
        self.write_register(Registers::GLOBALSCALER, &mut value)
    }

    /// write value to TPOWERDOWN register
    pub fn set_tpowerdown(&mut self, val: u32) -> Result<DataPacket, Error<E>> {
        let mut value = val.to_be_bytes();
        self.write_register(Registers::TPOWERDOWN, &mut value)
    }

    /// write value to TPWMTHRS register
    pub fn set_tpwmthrs(&mut self, val: u32) -> Result<DataPacket, Error<E>> {
        let mut value = val.to_be_bytes();
        self.write_register(Registers::TPWMTHRS, &mut value)
    }

    /// write value to TCOOLTHRS register
    pub fn set_tcoolthrs(&mut self, val: u32) -> Result<DataPacket, Error<E>> {
        let mut value = val.to_be_bytes();
        self.write_register(Registers::TCOOLTHRS, &mut value)
    }

    /// write value to A1 register
    pub fn set_a1(&mut self, val: u32) -> Result<DataPacket, Error<E>> {
        let mut value = val.to_be_bytes();
        self.write_register(Registers::A1, &mut value)
    }

    /// write value to V1 register
    pub fn set_v1(&mut self, val: u32) -> Result<DataPacket, Error<E>> {
        let mut value = val.to_be_bytes();
        self.write_register(Registers::V1, &mut value)
    }

    /// write value to AMAX register
    pub fn set_amax(&mut self, val: u32) -> Result<DataPacket, Error<E>> {
        let mut value = val.to_be_bytes();
        self.write_register(Registers::AMAX, &mut value)
    }

    /// write value to VMAX register
    pub fn set_vmax(&mut self, val: u32) -> Result<DataPacket, Error<E>> {
        let mut value = val.to_be_bytes();
        self.write_register(Registers::VMAX, &mut value)
    }

    /// write value to DMAX register
    pub fn set_dmax(&mut self, val: u32) -> Result<DataPacket, Error<E>> {
        let mut value = val.to_be_bytes();
        self.write_register(Registers::DMAX, &mut value)
    }

    /// write value to D1 register
    pub fn set_d1(&mut self, val: u32) -> Result<DataPacket, Error<E>> {
        let mut value = val.to_be_bytes();
        self.write_register(Registers::D1, &mut value)
    }

    /// write value to VSTART register
    pub fn set_vstart(&mut self, val: u32) -> Result<DataPacket, Error<E>> {
        let mut value = val.to_be_bytes();
        self.write_register(Registers::VSTART, &mut value)
    }

    /// write value to VSTOP register
    pub fn set_vstop(&mut self, val: u32) -> Result<DataPacket, Error<E>> {
        let mut value = val.to_be_bytes();
        self.write_register(Registers::VSTOP, &mut value)
    }

    /// write value to PWM_AUTO register
    pub fn set_pwm_auto(&mut self, val: u32) -> Result<DataPacket, Error<E>> {
        let mut value = val.to_be_bytes();
        self.write_register(Registers::PWM_AUTO, &mut value)
    }

    /// write value to RAMPMODE register
    pub fn set_rampmode(&mut self, val: RampMode) -> Result<DataPacket, Error<E>> {
        let mut value = (val as u32).to_be_bytes();
        self.write_register(Registers::VSTOP, &mut value)
    }

    /// read GLOBALSCALER register
    pub fn read_global_scaler(&mut self) -> Result<u32, Error<E>> {
        self.read_register(Registers::GLOBALSCALER).map(|packet| packet.data)
    }

    /// read offset register
    pub fn read_offset(&mut self) -> Result<u32, Error<E>> {
        self.read_register(Registers::OFFSET_READ).map(|packet| packet.data)
    }

    /// read TSTEP register
    pub fn read_tstep(&mut self) -> Result<u32, Error<E>> {
        self.read_register(Registers::TSTEP).map(|packet| packet.data)
    }

    /// read DRV_STATUS register
    pub fn read_drv_status(&mut self) -> Result<DrvStatus, Error<E>> {
        let packet = self.read_register(Registers::DRV_STATUS)?;
        self.status = packet.status;
        Ok(DrvStatus::from_bytes(packet.data.to_be_bytes()))
    }

    /// read GSTAT register
    pub fn read_gstat(&mut self) -> Result<GStat, Error<E>> {
        let packet = self.read_register(Registers::GSTAT)?;
        self.status = packet.status;
        Ok(GStat::from_bytes(packet.data.to_be_bytes()))
    }

    /// read GCONF register
    pub fn read_gconf(&mut self) -> Result<GConf, Error<E>> {
        let packet = self.read_register(Registers::GCONF)?;
        self.status = packet.status;
        Ok(GConf::from_bytes(packet.data.to_be_bytes()))
    }

    /// read DRV_STATUS register
    pub fn read_ramp_status(&mut self) -> Result<RampStat, Error<E>> {
        let packet = self.read_register(Registers::RAMP_STAT)?;
        self.status = packet.status;
        Ok(RampStat::from_bytes(packet.data.to_be_bytes()))
    }

    /// set the position to 0 / home
    pub fn set_home(&mut self) -> Result<DataPacket, Error<E>> {
        let mut val = 0_u32.to_be_bytes();
        self.write_register(Registers::XACTUAL, &mut val)?;
        let packet = self.write_register(Registers::XTARGET, &mut val)?;
        self.status = packet.status;
        Ok(packet)
    }

    /// stop the motor now
    pub fn stop(&mut self) -> Result<DataPacket, Error<E>> {
        self.disable()?;
        let mut val = 0_u32.to_be_bytes();
        self.write_register(Registers::VSTART, &mut val)?;
        let packet = self.write_register(Registers::VMAX, &mut val)?;
        self.status = packet.status;
        Ok(packet)
    }

    /// check if the motor is moving
    pub fn is_moving(&mut self) -> Result<bool, Error<E>> {
        self.read_drv_status().map(|packet| !packet.standstill())
    }

    /// check if motor is at right limit
    pub fn is_at_limit_r(&mut self) -> Result<bool, Error<E>> {
        self.read_ramp_status().map(|packet| packet.status_stop_r())
    }

    /// check if motor is at left limit
    pub fn is_at_limit_l(&mut self) -> Result<bool, Error<E>> {
        self.read_ramp_status().map(|packet| packet.status_stop_l())
    }

    /// set the max velocity (VMAX)
    pub fn set_velocity(&mut self, velocity: f32) -> Result<DataPacket, Error<E>> {
        self.v_max = velocity;
        let v_max = self.speed_from_hz(velocity);
        let mut val = v_max.to_be_bytes();
        let packet = self.write_register(Registers::VMAX, &mut val)?;
        self.status = packet.status;
        Ok(packet)
    }

    /// set the max acceleration (AMAX, DMAX, A1, D1)
    pub fn set_acceleration(&mut self, acceleration: f32) -> Result<DataPacket, Error<E>> {
        let a_max = self.accel_from_hz(acceleration);
        let mut val = a_max.to_be_bytes();
        self.write_register(Registers::AMAX, &mut val)?;
        self.write_register(Registers::DMAX, &mut val)?;
        self.write_register(Registers::A1, &mut val)?;
        let packet = self.write_register(Registers::D1, &mut val)?;
        self.status = packet.status;
        Ok(packet)
    }

    /// move to a specific location
    pub fn move_to(&mut self, target_signed: i32) -> Result<DataPacket, Error<E>> {
        self.enable()?;
        let target = (target_signed * self._step_count as i32) as u32;
        let mut val = target.to_be_bytes();
        let packet = self.write_register(Registers::XTARGET, &mut val)?;
        self.status = packet.status;
        Ok(packet)
    }

    /// get the current position
    pub fn get_position(&mut self) -> Result<f32, Error<E>> {
        self.read_register(Registers::XACTUAL).map(|val| val.data as f32 / self._step_count / 400.0)
    }

    /// set the current position
    pub fn set_position(&mut self, target_signed: i32) -> Result<DataPacket, Error<E>> {
        let target = target_signed as u32;
        let mut val = (target * self._step_count as u32).to_be_bytes();
        self.write_register(Registers::XACTUAL, &mut val)
    }

    /// get the current velocity
    pub fn get_velocity(&mut self) -> Result<f32, Error<E>> {
        self.read_register(Registers::VACTUAL).map(|target| {
            if (target.data & 0b100000000000000000000000) == 0b100000000000000000000000 {
                ((16777216 - target.data as i32) as f64 / self._step_count as f64) as f32
            } else {
                ((target.data as i32) as f64 / self._step_count as f64) as f32
            }
        })
    }

    /// get the set maximum velocity (VMAX)
    pub fn get_velocity_max(&mut self) -> f32 {
        self.v_max / 400.0
    }

    /// get the current target position (XTARGET)
    pub fn get_target(&mut self) -> Result<i32, Error<E>> {
        self.read_register(Registers::XTARGET).map(|packet| packet.data as i32)
    }
}

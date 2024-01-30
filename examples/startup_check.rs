//! Test the formatted output
//!
//! This example hasn't a special requires
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

// required for the to_u32_le() function.
use modular_bitfield_to_value::ToValue;

use cortex_m::asm;
use cortex_m_rt::entry;
use serialio::{sprintln, SerialIO};
use stm32f7x7_hal::{
    prelude::*,
    serial::{config, Serial},
    spi::Spi,
    stm32,
};

use tmc5160::registers::*;
use tmc5160::{DataPacket, Error, Tmc5160};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let gpiod = p.GPIOD.split();
    let gpiob = p.GPIOB.split();
    let gpioa = p.GPIOA.split();

    // Setup serial i/o
    let tx = gpiod.pd8.into_alternate_af7();
    let rx = gpiod.pd9.into_alternate_af7();

    let conf = config::Config::default();
    let serial = Serial::usart3(p.USART3, (tx, rx), conf.baudrate(115_200.bps()), clocks);

    let (tx, rx) = serial.unwrap().split();

    let mut in_out = SerialIO::new(tx, rx);

    sprintln!(in_out, "Stepper driver startup check");

    // Setup spi i/o
    let sck = gpiob.pb3.into_alternate_af5();
    let miso = gpiob.pb4.into_alternate_af5();
    let mosi = gpiob.pb5.into_alternate_af5();
    let mut nss = gpioa.pa4.into_push_pull_output();
    nss.set_high();

    let spi = Spi::spi1(
        p.SPI1,
        (sck, miso, mosi),
        tmc5160::MODE,
        500.khz().into(),
        clocks,
    );

    // set up stepper driver
    let mut stepper_driver = Tmc5160::new(spi, nss);
    // optionally, you could attach an EN pin, which then lets you use the `enable()` and `disable()` functions:
    // let mut stepper_driver = Tmc5160::new(spi, nss).attach_en(en);
    // you could also invert this pin (normally not required):
    // let mut stepper_driver = Tmc5160::new(spi, nss).attach_en(en).en_inverted(true);

    // clear G_STAT register
    match stepper_driver.clear_g_stat(){
        Ok(packet) => {
            sprintln!(in_out, "SPI status has been updated: {}", packet.status.to_u32_le().unwrap_or(0));
        }
        Err(error) => {
            sprintln!(in_out, "Error clearing GSTAT is {:?}", error);
        }
    }

    // read OFFSET
    match stepper_driver.read_offset() {
        Ok(offset) => {
            sprintln!(in_out, "Stepper driver offset is {}", offset);
        }
        Err(error) => {
            sprintln!(in_out, "Error for reading offset is {:?}", error);
        }
    }

    // set G_CONF register
    stepper_driver
        .g_conf
        .set_recalibrate(true)
        .set_faststandstill(true)
        .set_en_pwm_mode(true);
    match stepper_driver.update_g_conf(){
        Ok(packet) => {
            sprintln!(in_out, "SPI status has been updated: {}", packet.status.to_u32_le().unwrap_or(0));
        }
        Err(error) => {
            sprintln!(in_out, "Error for updating GCONF is {:?}", error);
        }
    }

    match stepper_driver.read_drv_status() {
        Ok(status) => {
            // either use fields of the register
            sprintln!(in_out, "Stepper driver is in standstill: {}", status.standstill());
            // or extract the u32 value from the register
            sprintln!(in_out, "Stepper driver DRV_STATUS register is {}", status.to_u32_le().unwrap_or(0));
            sprintln!(in_out, "SPI status has been updated: {}", stepper_driver.status.to_u32_le().unwrap_or(0));
        }
        Err(error) => {
            sprintln!(in_out, "Error for reading DRV_STATUS is {:?}", error);
        }
    }

    match stepper_driver.read_gstat() {
        Ok(status) => {
            sprintln!(in_out, "Stepper GSTAT register is {}", status.to_u32_le().unwrap_or(0));
            sprintln!(in_out, "SPI status has been updated: {}", stepper_driver.status.to_u32_le().unwrap_or(0));
        }
        Err(error) => {
            sprintln!(in_out, "Error for reading GSTAT is {:?}", error);
        }
    }

    // ...

    asm::bkpt();

    loop {}
}

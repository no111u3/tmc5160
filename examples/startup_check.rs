//! Test the formated output
//!
//! This example hasn't a special requires
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

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

    // clear G_STAT register
    stepper_driver.clear_g_stat()?;

    // read OFFSET
    match stepper_driver.read_offset() {
        Ok(offset) => {
            sprintln!(in_out, "Stepper driver offset is {}", offset);
        }
        Err(error) => {
            sprintln!(in_out, "Error for read status is {:?}", error);
        }
    }

    // set G_CONF register
    stepper_driver
        .g_conf
        .with_recalibrate(true)
        .with_faststandstill(true)
        .with_en_pwm_mode(true);
    stepper_driver.update_g_conf()?;

    match stepper_driver.read_drv_status() {
        Ok(status) => {
            // either use fields of the register
            sprintln!(in_out, "Stepper driver is in standstill: {}", status);
            // or extract the u32 value from the register
            sprintln!(in_out, "Stepper driver DRV_STATUS register is {}", status.to_u32());
            sprintln!(in_out, "SPI status has been updated: {}", stepper_driver.status);
        }
        Err(error) => {
            sprintln!(in_out, "Error for read status is {:?}", error);
        }
    }

    match stepper_driver.read_gstat() {
        Ok(status) => {
            sprintln!(in_out, "Stepper GSTAT register is {}", status.to_u32());
            sprintln!(in_out, "SPI status has been updated: {}", stepper_driver.status);
        }
        Err(error) => {
            sprintln!(in_out, "Error for read status is {:?}", error);
        }
    }

    // ...

    asm::bkpt();

    loop {}
}

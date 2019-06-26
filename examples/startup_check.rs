//! Test the formated output
//!
//! This example hasn't a special requires
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m::asm;
use stm32f7x7_hal::{stm32, prelude::*, serial::{Serial, config}, spi::Spi};
use serialio::{SerialIO, sprintln};
use cortex_m_rt::entry;

use tmc5160::{self, Tmc5160};

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

    let spi = Spi::spi1(p.SPI1, (sck, miso, mosi), tmc5160::MODE, 500.khz().into(), clocks);

    let mut stepper_driver = Tmc5160::new(spi, nss).unwrap();
    
    match stepper_driver.read_register(tmc5160::Registers::GCONF) {
        Ok(conf) => {sprintln!(in_out, "Stepper driver conf is {}", conf);},
        Err(error) => {sprintln!(in_out, "Error for read status is {:?}", error);},
    }
    
    match stepper_driver.read_register(tmc5160::Registers::GCONF) {
        Ok(conf) => {sprintln!(in_out, "Stepper driver conf is {}", conf);},
        Err(error) => {sprintln!(in_out, "Error for read status is {:?}", error);},
    }

    match stepper_driver.read_register(tmc5160::Registers::GSTAT) {
        Ok(status) => {
            sprintln!(in_out, "Stepper driver status is {}", status);},
        Err(error) => {sprintln!(in_out, "Error for read status is {:?}", error);},
    }
    
    match stepper_driver.read_register(tmc5160::Registers::GSTAT) {
        Ok(status) => {
            sprintln!(in_out, "Stepper driver status is {}", status);},
        Err(error) => {sprintln!(in_out, "Error for read status is {:?}", error);},
    }
    
    match stepper_driver.read_register(tmc5160::Registers::INP_OUT) {
        Ok(status) => {
            sprintln!(in_out, "Stepper driver status is {}", status);},
        Err(error) => {sprintln!(in_out, "Error for read status is {:?}", error);},
    }
    
    match stepper_driver.read_register(tmc5160::Registers::INP_OUT) {
        Ok(status) => {
            sprintln!(in_out, "Stepper driver status is {}", status);},
        Err(error) => {sprintln!(in_out, "Error for read status is {:?}", error);},
    }

    asm::bkpt();

    loop {}
}

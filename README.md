# Trinamic TMC5160 Rust Driver

[![crates.io](https://img.shields.io/crates/v/tmc5160.svg)](https://crates.io/crates/tmc5160)
[![Docs](https://docs.rs/tmc5160/badge.svg)](https://docs.rs/tmc5160)
[![Rust](https://github.com/hacknus/tmc5160-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/hacknus/tmc5160-rs/actions/workflows/rust.yml)

Platform agnostic rust driver for the [TMC5160](https://www.trinamic.com/fileadmin/assets/Products/ICs_Documents/TMC5160A_datasheet_rev1.18.pdf) Trinamic integrated stepper motor controller.

To implement this driver, consult the example:

```rust
use tmc5160::registers::*;
use tmc5160::{DataPacket, Error, Tmc5160};

{
    // set up spi ...

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
    match stepper_driver.update_g_conf(){
        Ok(packet) => {
            sprintln!(in_out, "SPI status has been updated: {}", packet.status);
        }
        Err(error) => {
            sprintln!(in_out, "Error for read status is {:?}", error);
        }
    }

    match stepper_driver.read_drv_status() {
        Ok(status) => {
            // either use fields of the register
            sprintln!(in_out, "Stepper driver is in standstill: {}", status);
            // or extract the u32 value from the register
            let array = status.into_bytes();
            let status = ((array[0] as u32) << 24)
                + ((array[1] as u32) << 16)
                + ((array[2] as u32) << 8)
                + ((array[3] as u32) << 0);
            sprintln!(in_out, "Stepper driver DRV_STATUS register is {}", status);
            sprintln!(in_out, "SPI status has been updated: {}", stepper_driver.status);
        }
        Err(error) => {
            sprintln!(in_out, "Error for read status is {:?}", error);
        }
    }

    match stepper_driver.read_gstat() {
        Ok(status) => {
            let array = status.into_bytes();
            let status = ((array[0] as u32) << 24)
                + ((array[1] as u32) << 16)
                + ((array[2] as u32) << 8)
                + ((array[3] as u32) << 0);
            sprintln!(in_out, "Stepper GSTAT register is {}", status);
            sprintln!(in_out, "SPI status has been updated: {}", stepper_driver.status);
        }
        Err(error) => {
            sprintln!(in_out, "Error for read status is {:?}", error);
        }
    }
}
```
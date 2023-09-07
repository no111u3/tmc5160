# Trinamic TMC5160 Rust Driver

[![crates.io](https://img.shields.io/crates/v/tmc5160.svg)](https://crates.io/crates/tmc5160)
[![Docs](https://docs.rs/tmc5160/badge.svg)](https://docs.rs/tmc5160)
[![Rust](https://github.com/hacknus/tmc5160-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/hacknus/tmc5160-rs/actions/workflows/rust.yml)

Platform agnostic rust driver for the [TMC5160](https://www.trinamic.com/fileadmin/assets/Products/ICs_Documents/TMC5160A_datasheet_rev1.18.pdf) Trinamic integrated stepper motor controller.

To implement this driver, consult the example:
```rust
use tmc5160::Tmc5160;

{
    let mut stepper_driver = Tmc5160::new(spi, nss).unwrap();

    match stepper_driver.read_register(tmc5160::Registers::GCONF) {
        Ok(conf) => {
            sprintln!(in_out, "Stepper driver conf is {}", conf);
        }
        Err(error) => {
            sprintln!(in_out, "Error for read status is {:?}", error);
        }
    }

    match stepper_driver.read_register(tmc5160::Registers::GSTAT) {
        Ok(status) => {
            sprintln!(in_out, "Stepper driver status is {}", status);
        }
        Err(error) => {
            sprintln!(in_out, "Error for read status is {:?}", error);
        }
    }

    match stepper_driver.read_register(tmc5160::Registers::INP_OUT) {
        Ok(status) => {
            sprintln!(in_out, "Stepper driver status is {}", status);
        }
        Err(error) => {
            sprintln!(in_out, "Error for read status is {:?}", error);
        }
    }
}
```
//cargo flash --chip stm32f411ceux --release

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let gpiob = p.GPIOB.split();
    let mut led = gpiob.pb4.into_push_pull_output();
    let mut i = 0;

    loop {
        //led.set_high();
        for _ in 0..1_000_000 {
            led.set_low();
            i = i + 1;
        }

        for _ in 0..1_000_000 {
            led.set_high();
        }
    }
}
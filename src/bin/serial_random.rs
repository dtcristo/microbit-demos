#![no_main]
#![no_std]

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;
use cortex_m_rt::entry;
use nrf51::{Peripherals, RNG, UART0};
use nrf51_hal::prelude::*;
use nrf51_hal::serial::{Serial, Tx, BAUD115200};
use panic_halt;

#[entry]
fn main() -> ! {
    if let Some(p) = Peripherals::take() {
        let gpio = p.GPIO.split();
        let tx_pin = gpio.pin24.into_push_pull_output().downgrade();
        let rx_pin = gpio.pin25.into_floating_input().downgrade();
        let (mut tx, _) = Serial::uart0(p.UART0, tx_pin, rx_pin, BAUD115200).split();

        // Enable error correction for better values
        p.RNG.config.write(|w| w.dercen().enabled());
        // Enable random number generation
        p.RNG.tasks_start.write(|w| unsafe { w.bits(1) });

        loop {
            // Wait for a new random value
            while p.RNG.events_valrdy.read().bits() == 0 {}
            // Read byte from the RNG
            let random_byte = p.RNG.value.read().bits() as u8;
            // Send byte over serial
            write!(tx, "{:x}", random_byte).unwrap();
            // Clear event for next random value
            p.RNG.events_valrdy.write(|w| unsafe { w.bits(0) });
        }
    }
    panic!();
}

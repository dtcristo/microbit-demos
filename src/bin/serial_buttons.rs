#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use nrf51::Peripherals;
use nrf51_hal::prelude::*;
use nrf51_hal::serial::{Serial, BAUD115200};
use panic_halt;

#[entry]
fn main() -> ! {
    if let Some(p) = Peripherals::take() {
        let gpio = p.GPIO.split();
        let tx_pin = gpio.pin24.into_push_pull_output().downgrade();
        let rx_pin = gpio.pin25.into_floating_input().downgrade();
        let (mut tx, _) = Serial::uart0(p.UART0, tx_pin, rx_pin, BAUD115200).split();
        let button_a = gpio.pin17.into_floating_input();
        let button_b = gpio.pin26.into_floating_input();
        let mut state_a_low = false;
        let mut state_b_low = false;
        loop {
            let button_a_low = button_a.is_low();
            let button_b_low = button_b.is_low();
            if button_a_low && !state_a_low {
                write!(tx, "A down\n\r").unwrap();
            }
            if button_b_low && !state_b_low {
                write!(tx, "B down\n\r").unwrap();
            }
            if !button_a_low && state_a_low {
                write!(tx, "A up\n\r").unwrap();
            }
            if !button_b_low && state_b_low {
                write!(tx, "B up\n\r").unwrap();
            }
            state_a_low = button_a_low;
            state_b_low = button_b_low;
        }
    }
    panic!();
}

#![no_main]
#![no_std]

use panic_halt;

use core::fmt::Write;

use rt::entry;

use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;
use microbit::hal::serial;

use microbit::led;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        let gpio = p.GPIO.split();
        let mut delay = Delay::new(p.TIMER0);

        let col1 = gpio.pin4.into_push_pull_output();
        let col2 = gpio.pin5.into_push_pull_output();
        let col3 = gpio.pin6.into_push_pull_output();
        let col4 = gpio.pin7.into_push_pull_output();
        let col5 = gpio.pin8.into_push_pull_output();
        let col6 = gpio.pin9.into_push_pull_output();
        let col7 = gpio.pin10.into_push_pull_output();
        let col8 = gpio.pin11.into_push_pull_output();
        let col9 = gpio.pin12.into_push_pull_output();
        let row1 = gpio.pin13.into_push_pull_output();
        let row2 = gpio.pin14.into_push_pull_output();
        let row3 = gpio.pin15.into_push_pull_output();
        let mut leds = led::Display::new(
            col1, col2, col3, col4, col5, col6, col7, col8, col9, row1, row2, row3,
        );

        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();
        let (mut tx, _) = serial::Serial::uart0(p.UART0, tx, rx, serial::BAUD115200).split();

        let _ = write!(tx, "\n\rStarting...\n\r");

        let checker_a = [
            [1, 0, 1, 0, 1],
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
        ];

        let checker_b = [
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [0, 1, 0, 1, 0],
        ];

        loop {
            write!(tx, "Hello micro:bit!\n\r").ok();
            leds.display(&mut delay, checker_a, 1000);
            leds.display(&mut delay, checker_b, 1000);
        }
    }
    panic!();
}
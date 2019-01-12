#![no_main]
#![no_std]

use panic_halt;

use cortex_m_rt::entry;

use microbit::hal::prelude::*;
use microbit::hal::serial;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        let mut gpio = p.GPIO.split();
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();
        let (mut tx, mut rx) = serial::Serial::uart0(p.UART0, tx, rx, serial::BAUD115200).split();
        loop {
            let val = microbit::block!(rx.read()).unwrap();
            microbit::block!(tx.write(val));
        }
    }
    panic!();
}

#![no_main]
#![no_std]

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::Peripherals as CorePeripherals;
use cortex_m_rt::entry;
use nrf51::{interrupt, Interrupt, Peripherals, GPIOTE, NVIC, UART0};
use nrf51_hal::prelude::*;
use nrf51_hal::serial::{Serial, Tx, BAUD115200};
use panic_halt;

static GPIOTE: Mutex<RefCell<Option<GPIOTE>>> = Mutex::new(RefCell::new(None));
static TX: Mutex<RefCell<Option<Tx<UART0>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let (Some(p), Some(mut cp)) = (Peripherals::take(), CorePeripherals::take()) {
        cortex_m::interrupt::free(|cs| {
            let gpio = p.GPIO.split();
            let tx_pin = gpio.pin24.into_push_pull_output().downgrade();
            let rx_pin = gpio.pin25.into_floating_input().downgrade();
            let (tx, _) = Serial::uart0(p.UART0, tx_pin, rx_pin, BAUD115200).split();
            let _ = gpio.pin17.into_floating_input();
            let _ = gpio.pin26.into_floating_input();

            // Configure button A interrupt when pressed
            p.GPIOTE.config[0]
                .write(|w| unsafe { w.mode().event().psel().bits(17).polarity().hi_to_lo() });
            p.GPIOTE.intenset.write(|w| w.in0().set());
            p.GPIOTE.events_in[0].write(|w| unsafe { w.bits(0) });

            // Configure button B interrupt when pressed
            p.GPIOTE.config[1]
                .write(|w| unsafe { w.mode().event().psel().bits(26).polarity().hi_to_lo() });
            p.GPIOTE.intenset.write(|w| w.in1().set());
            p.GPIOTE.events_in[1].write(|w| unsafe { w.bits(0) });

            // Configure button A interrupt when released
            p.GPIOTE.config[2]
                .write(|w| unsafe { w.mode().event().psel().bits(17).polarity().lo_to_hi() });
            p.GPIOTE.intenset.write(|w| w.in2().set());
            p.GPIOTE.events_in[2].write(|w| unsafe { w.bits(0) });

            // Configure button B interrupt when released
            p.GPIOTE.config[3]
                .write(|w| unsafe { w.mode().event().psel().bits(26).polarity().lo_to_hi() });
            p.GPIOTE.intenset.write(|w| w.in3().set());
            p.GPIOTE.events_in[3].write(|w| unsafe { w.bits(0) });

            // Store peripherals in Mutex for use within interrupt
            GPIOTE.borrow(cs).replace(Some(p.GPIOTE));
            TX.borrow(cs).replace(Some(tx));

            // Enable interrupts for GPIO
            cp.NVIC.enable(Interrupt::GPIOTE);
            NVIC::unpend(Interrupt::GPIOTE);
        });

        loop {
            continue;
        }
    }
    panic!();
}

#[interrupt]
fn GPIOTE() {
    cortex_m::interrupt::free(|cs| {
        if let (Some(gpiote), Some(ref mut tx)) = (
            GPIOTE.borrow(cs).borrow().as_ref(),
            TX.borrow(cs).borrow_mut().deref_mut(),
        ) {
            if gpiote.events_in[0].read().bits() != 0 {
                write!(tx, "A down\n\r").unwrap();
                gpiote.events_in[0].write(|w| unsafe { w.bits(0) });
            }
            if gpiote.events_in[1].read().bits() != 0 {
                write!(tx, "B down\n\r").unwrap();
                gpiote.events_in[1].write(|w| unsafe { w.bits(0) });
            }
            if gpiote.events_in[2].read().bits() != 0 {
                write!(tx, "A up\n\r").unwrap();
                gpiote.events_in[2].write(|w| unsafe { w.bits(0) });
            }
            if gpiote.events_in[3].read().bits() != 0 {
                write!(tx, "B up\n\r").unwrap();
                gpiote.events_in[3].write(|w| unsafe { w.bits(0) });
            }
        }
    });
}

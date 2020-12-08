#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate feather_m0 as hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

fn onOffForTime(time: u32, delay: &mut Delay, red_led: &mut hal::gpio::Pin<feather_m0::gpio::v2::PA17, hal::gpio::v2::Output<hal::gpio::v2::PushPull>>) -> () {
    delay.delay_ms(time);
    red_led.set_high().unwrap();
    delay.delay_ms(time);
    red_led.set_low().unwrap();
}

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        for _ in 0..3 {
            onOffForTime(100u32, &mut delay, &mut red_led);
        }

        delay.delay_ms(400u32);

        for _ in 0..3 {
            onOffForTime(200u32, &mut delay, &mut red_led);
        }
        delay.delay_ms(400u32);        
        
    }
}

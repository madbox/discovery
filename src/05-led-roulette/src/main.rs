#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let half_period = 200_u16;

    loop {
        for x in 0..8 {
            leds[x].on();
            delay.delay_ms(half_period);
            if x > 0 {
                leds[x-1].off();
            } else {
                leds[7].off();
            }
        }
    }
}

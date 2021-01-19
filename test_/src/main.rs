#![no_std]
#![no_main]

use panic_halt; // require to start write any code at all
use cortex_m_rt::entry; // require to start write any code at all
use f3::hal::{prelude::*, stm32f30x}; // require to compile code for stm32f3
//use f3::led::Leds; -> later I have to check it out 
//use cortex_m::peripheral::Peripherals; -> later I have to check it out 
//use f3::led::Leds; -> later I have to check it out 

use stm32f3_discovery::leds::Leds;

#[entry]
fn main() -> ! {
    let mut x: u32 = 10;
    let _x = 20 + x;
    let mut y;

    y = &_x;

    let mut _leds = Leds::new();

    loop {}
}

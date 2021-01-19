#![no_std]
#![no_main]

use panic_halt;
use cortex_m_rt::entry;
use f3::hal::{prelude::*, stm32f30x};

#[entry]
fn main() -> ! {
    let mut x: u32 = 10;
    let _x = 20 + x;

    loop {}
}

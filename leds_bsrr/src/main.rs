#![no_std]
#![no_main]

use panic_halt;
use cortex_m_rt::entry;
use f3::hal::{ prelude::*, delay::Delay, stm32f30x::{self, GPIOE} };
use f3::{ hal::prelude::*, led::Leds } ;

#[entry]
fn main() -> ! {
	let cortex = cortex_m::Peripherals::take().unwrap();
	let stm32 = stm32f30x::Peripherals::take().unwrap();

	let mut rcc = stm32.RCC.constrain();

	let mut flash = stm32.FLASH.constrain();

	let time = rcc.cfgr.freeze(&mut flash.acr);

	let mut delay = Delay::new(cortex.SYST, time);

	Leds::new(stm32.GPIOE.split(&mut rcc.ahb)); // new Leds instance for doing stuff with Leds it is required

	// You have two ways of doing stuff in low level programming in rust 
	// You can do it by a api of GPIOE like below or by address of BSRR
	// Aproache one with api of GPIOE
	let gpioe = unsafe{ &*GPIOE::ptr() };

	gpioe.bsrr.write(|w| w.bs9().set_bit());
	delay.delay_ms(400u32);
	gpioe.bsrr.write(|w| w.bs10().set_bit());
	delay.delay_ms(400u32);
	gpioe.bsrr.write(|w| w.br10().set_bit());
	delay.delay_ms(400u32);
	gpioe.bsrr.write(|w| w.br9().set_bit());

	// Aproache two with address of BSRR
		unsafe {
			const GPIOE_BSRR: u32 = 0x48001018; // You can find that address in manual of board

			for i in 8..14 {
				*(GPIOE_BSRR as *mut u32) = 1 << &i;
				delay.delay_ms(300u32);
				*(GPIOE_BSRR as *mut u32) = 1 << (i + 16);
			}
		}

	loop { }

}
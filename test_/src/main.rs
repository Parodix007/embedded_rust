#![no_std]
#![no_main]

use panic_halt; // require to start write any code at all
use cortex_m_rt::entry; // require to start write any code at all
use f3::{hal::prelude::*, 
		led::Leds,
		}; // require to compile code for stm32f3
use f3::hal::{prelude::*, stm32f30x, delay::Delay }; // require to deal with stuff of stm32f3 board

#[entry]
fn main() -> ! {
	// Random basic stuff
    let mut x: u32 = 10;
    let _x = 20 + x;
    let mut y;

    y = &_x;

    // Trying figure out stuff with Leds
    let stm = stm32f30x::Peripherals::take().unwrap(); // take all peripherals of stm32f3 in singleton paradigm
    let cortex = cortex_m::Peripherals::take().unwrap(); // take all peripherals of cortex_m microcontroller in singleton paradigm
    let mut rcc = stm.RCC.constrain(); // take RCC from stm32f3 (reset and clock control) and use constrain method do deal with it basiclly it do something with the pins on board
    let mut flash = stm.FLASH.constrain(); // just flash idk

    let time = rcc.cfgr.freeze(&mut flash.acr); // for dealing with errors that may ocure; cfgr -> Clock configuration register; idk what is freeze to catch time when error ocur maybe 

    let mut leds = Leds::new(stm.GPIOE.split(&mut rcc.ahb)); // use RCC as parametr for all the require pins
    let mut delay = Delay::new(cortex.SYST, time); // creating new dealy obj

    let mut seq_even = 0;
    let mut seq_odd = 0;
    let _delay: u32 = 200;

      unsafe {
        const GPIOE_BSRR: u32 = 0x48001018;

        *(GPIOE_BSRR as *mut u32) = 1 << 9; // on led
        *(GPIOE_BSRR as *mut u32) = 1 << 11; // on led
        *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16); // off led
        *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16); // off led
            
    }
    
    loop {
        // for i in 0..leds.iter().len() {
     //        if seq_even > 5 {
     //            if i % 2 != 0 {
     //                leds[i].on();
     //                delay.delay_ms(_delay);
     //            }
     //        } else {
     //            if i % 2 == 0 {
     //    			leds[i].on();
     //    			delay.delay_ms(_delay);
     //            }
    	// 	}
    	// }

     //    delay.delay_ms(300u32);

     //    for i in 0..leds.iter().len() {
     //        delay.delay_ms(_delay);
     //        leds[i].off();
     //    }

     //    if seq_even <= 5 {
     //        seq_even+=1;
     //    }else {
     //        if seq_odd <= 5 {
     //            seq_odd+=1;
     //        } else {
     //            seq_even = 0;
     //            seq_odd = 0;
     //        }
     //    }
    	
    }
}

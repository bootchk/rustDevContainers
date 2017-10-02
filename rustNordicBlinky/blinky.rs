// examples/blinky.rs

#![feature(used)]
#![no_std]

// version = "0.2.0", default-features = false
// extern crate cast;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate nordic;

// use core::u16;

// use cast::{u16, u32};
use cortex_m::asm;
use nordic::{ RTC0, CLOCK, GPIOTE };  // , P0

//mod frequency {
    // Frequency of APB1 bus (TIM7 is connected to this bus)
    // pub const APB1: u32 = 8_000_000;
//}

/// Timer frequency
//const FREQUENCY: u32 = 1;


// For all "task" register operations
// Per product spec, write "1" to task register
// The tasks run on the peripheral, concurrently.
// Returning from starting a task does not guarantee that the task has completed.

// For all "event" register ops
// Is 1 when event has occurred.
// Writing 0 clears register.

pub fn start_lfclock() {
	// Per product spec, write "1" to task register
	
    // NOTE(safe) atomic write
    unsafe { (*CLOCK.get()).tasks_lfclkstart.write(|w| w.bits(1)) }
}

pub fn start_rtc_counter() {
    // NOTE(safe) atomic write
    unsafe { (*RTC0.get()).tasks_start.write(|w| w.bits(1)) }
}


pub fn is_rtc_compare0_match() -> bool {
    unsafe { (*RTC0.get()).events_compare0.read().bits() == 1 }
    // [0]
}

pub fn clear_rtc0_compare0_event() {
	unsafe { (*RTC0.get()).events_compare0.write(|w| w.bits(0)) }
}


pub fn set_rtc0_compare0() {
	unsafe { (*RTC0.get()).cc0.write(|w| w.bits(100000)) }
}

// Set compare reg in advance of counter by x
// Hack, small x (less than 2) won't work because of hw limitation (event will come late, after overflow)
// Hack, increment is hard coded
pub fn set_rtc0_compare0_past_counter() {
	// Counter is 24-bit, with clock semantics (circular, overflows back to 0).
	unsafe { 
		let now = (*RTC0.get()).counter.read().bits();
	    // This should be modulo 24-bit arithmetic
	    // But it works because the hw masks off all but 24-bits
	    let new_compare = now + 100000;
	    (*RTC0.get()).cc0.write(|w| w.bits(new_compare));
    }
}



// Note that Nordic docs refer to these as GPIO and GPIOTE, 
// but the GPIO is renamed P0 in the svd.
// Note there are registers for each pin, and also registers that are bit-fields by pin

//pub fn configure_gpio_pin_out() {
    // NOTE(safe) atomic write
//    unsafe { (*P0.get()).pin_cnf0.dir.write(|w| w.bits(1)) }
//}

//pub fn set_gpio_pin_high(){
    // NOTE(safe) atomic write
//    unsafe { (*GPIOTE.get()).pin_cnf0.write(|w| w.bits(1)) }
//}
//pub fn set_gpio_pin_low() {
    // NOTE(safe) atomic write
//    unsafe { (*GPIOTE.get()).pin_cnf0.write(|w| w.bits(1)) }
//}


// Configure GPIOTE channel so a task will toggle a pin configured as output.
// A HW task triggered through a register in the GPIOTE
// The HW remembers the state of the pin, and HW implements toggle
pub fn configure_gpiote_to_toggle_pin() {
    // NOTE(safe) atomic write
    
    // Notes on syntax:
    // config0 is the register  (i.e. CONFIG[0])
    // the write closure takes method calls that return(identify) fields of the register 
    // e.g. w.mode() means the MODE field
    // the field selector takes method calls that return(identify) an enumerated value for the field 
    // e,g, w.mode().task() means set MODE filed to TASK value
    
    // Select pin for this channel
    // Value to write is [0..31] the index of the pin
    // Here, pin 
    unsafe { (*GPIOTE.get()).config0.write(|w| w.psel().bits(1)) }
    // Pin is out and triggered by task
    unsafe { (*GPIOTE.get()).config0.write(|w| w.mode().task()) }
    // toggle (wherease SET means high and CLEAR means low)
    unsafe { (*GPIOTE.get()).config0.write(|w| w.polarity().toggle()) }
}

// Toggle LED by triggering preconfigured task register
pub fn toggle_led() {
    // NOTE(safe) atomic write
    unsafe { (*GPIOTE.get()).tasks_out0.write(|w| w.bits(1)) }
}



pub fn initialize_peripherals() {
	// Non-exclusive use of peripherals.
	// We don't expect any interrupts
	
	// Power up peripherals
    // nrf52 POR condition:
    // RTC's clock source is LFCLK
    // LFCLK source is LFRC (internal RC osc.)
    // LFCLK not started
    // RTC prescaler is none
    // LFCLK is 32khz i.e. 30uSec/tick
    // GPIO pins configured as input
    
    // Given using RTC, then either need to start LFCLK, or change RTC clock source to HFCLK, 
    // which is running whenever the cpu is not asleep
    
    start_lfclock();
    start_rtc_counter();
    // we don't wait, but eventually clock and counter will start and blinking will occur

    //
    configure_gpiote_to_toggle_pin();
    
    // TODO configure compare

}


#[inline(never)]
fn main() {

    initialize_peripherals();
    // assert counter is running
    // assert compare register not set
    
    // Configure pin as output
    // configure_gpio_pin_out();
    //gpioe.moder.modify(|_, w| w.moder9().output());

    // Configure TIM7 for periodic timeouts
    //let ratio = frequency::APB1 / FREQUENCY;
    //let psc = u16((ratio - 1) / u32(u16::MAX)).unwrap();
    //tim7.psc.write(|w| w.psc().bits(psc));
    //let arr = u16(ratio / u32(psc + 1)).unwrap();
    //tim7.arr.write(|w| w.arr().bits(arr));
    //tim7.cr1.write(|w| w.opm().continuous());

    // Start the timer
    //tim7.cr1.modify(|_, w| w.cen().enabled());


    set_rtc0_compare0_past_counter();
    // assert event will occur
    
    // app logic: loop, waiting on timer event, toggling led
    loop {
        // Busy wait for event
        while ! is_rtc_compare0_match() {}
		// rtc0.events_compare0.read().uif().is_no_update
		
        clear_rtc0_compare0_event();
        //tim7.sr.modify(|_, w| w.uif().clear());

		// Toggle LED
		toggle_led();
		
		set_rtc0_compare0_past_counter();
		// assert event will occur again
    }
}

// This part is the same as before
#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}

// 
// Derived from cortex-m-quickstart repository examples/blinky.rs
//
// Retargeted to Nordic nrf52 (probably would work for nrf51).

#![feature(used)]
#![no_std]

// version = "0.2.0", default-features = false
// extern crate cast;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate nordic;

use core::u8;

// use cast::{u16, u32};
use cortex_m::asm;	// see default handler
use nordic::{ RTC0, CLOCK, GPIOTE, P0 };  // , P0


// at 30uSec/tick, 1e5 is about 3 seconds
const TICKS_BETWEEN_TOGGLE: u32 = 100000;
// NRF52DK dev kit has 4 LEDS total, 17, 18, 19, 20
// type is u8 but value should not exceed 31 on this architecture
const LED_PIN_INDEX: u8 = 17;	


// For all "task" register operations
// Per product spec, write "1" to task register
// The tasks run on the peripheral, concurrently.
// Returning from starting a task does not guarantee that the task has completed.

// For all "event" register ops
// Is 1 when event has occurred.
// Writing 0 clears register.

// NOTE   Most below "unsafe" code is stipulated (safe) atomic write

pub fn start_lfclock() {
    
    unsafe { (*CLOCK.get()).tasks_lfclkstart.write(|w| w.bits(1)) }
}

pub fn start_rtc_counter() {
    unsafe { (*RTC0.get()).tasks_start.write(|w| w.bits(1)) }
}


pub fn is_rtc_compare0_event() -> bool {
	// should be 1 on event, but allow any bits set
    unsafe { (*RTC0.get()).events_compare0.read().bits() != 0 }
}

pub fn clear_rtc0_compare0_event() {
	unsafe { (*RTC0.get()).events_compare0.write(|w| w.bits(0)) }
}


//pub fn set_rtc0_compare0() {
//	unsafe { (*RTC0.get()).cc0.write(|w| w.bits(TICKS_BETWEEN_TOGGLE)) }
//}

// Set compare reg in advance of counter by x
// AKA set alarm on a clock

// This is a hack, NOT correct:
// In both cases, event may come late, after overflow (clock wrap)
// 1. small x (less than 2 or 3, see HW datasheet) won't work because of hw limitation 
// 2. need Lamport's rule to deal with overflow between read counter and set compare

pub fn set_rtc0_compare0_past_counter() {
	// Counter is 24-bit, with clock semantics (circular, overflows back to 0).
	unsafe { 
		let now = (*RTC0.get()).counter.read().bits();
		
	    // This should be modulo 24-bit arithmetic
	    // But it works because the hw masks off all but 24-bits
	    let new_compare = now + TICKS_BETWEEN_TOGGLE;
	    
	    (*RTC0.get()).cc0.write(|w| w.bits(new_compare));
    }
}



// Note that Nordic docs refer to these as GPIO and GPIOTE, 
// but the GPIO is renamed P0 in the svd.
// Note there are registers for each pin, and also registers that are bit-fields by pin

pub fn configure_gpio_pin_17_out() {
    unsafe { (*P0.get()).pin_cnf17.write(|w| w.dir().output()) }  // bits(1)) }
}

pub fn set_gpio_pin_high(){
    unsafe { (*P0.get()).out.write(|w| w.pin17().set_bit()) }
}
pub fn set_gpio_pin_low() {
    unsafe { (*P0.get()).out.write(|w| w.pin17().clear_bit()) }
}


// Configure GPIOTE channel so a task will toggle a pin configured as output.
// A HW task triggered through a register in the GPIOTE
// The HW knows state of pin, and HW implements toggle
pub fn configure_gpiote_to_toggle_pin() {
    // NOTE(safe) atomic write
    
    // Notes on syntax:
    // config0 is the config register for channel 0 (i.e. CONFIG[0])
    // the write closure takes method calls that return(identify) fields of the register 
    // e.g. w.mode() means the MODE field
    // the field selector takes method calls that return(identify) an enumerated value for the field 
    // e,g, w.mode().task() means set MODE filed to TASK value
    
    // Select pin for this channel
    // Value to write is [0..31] the index of the pin
    // i.e. a 5-bit number (not a 32-bit bitmask!!!)
    // bits() expects type u8
    unsafe { (*GPIOTE.get()).config0.write(|w| w.psel().bits(LED_PIN_INDEX)) }
    // Pin is out and triggered by start task
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
	
	// Start peripherals
	
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
}


#[inline(never)]
fn main() {

    initialize_peripherals();
    // assert counter is running
    // assert compare register not set
    
    // cruft from original stmf3 blinky
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

    // First toggle led using GPIO
    configure_gpio_pin_17_out();
    
    // low is on since pin sinks current  
    for x in 0..100000 {
         set_gpio_pin_low();
    }
    for x in 0..100000 {
         set_gpio_pin_high();
    }
    
    // Now use gpiote, which overrides GPIO setting
    configure_gpiote_to_toggle_pin();
    

    set_rtc0_compare0_past_counter();
    // assert event will occur
    
    toggle_led();
    
    // app logic: loop, waiting on timer event, toggling led
    loop {
        // Busy wait for event
        while ! is_rtc_compare0_event() {}
		// rtc0.events_compare0.read().uif().is_no_update
		
        clear_rtc0_compare0_event();
        //tim7.sr.modify(|_, w| w.uif().clear());

		toggle_led();
		
		set_rtc0_compare0_past_counter();
		// assert event will occur again
    }
}

// This part is the same as before
#[allow(dead_code)]
#[used]
#[link_section = ".vector_table.interrupts"]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}

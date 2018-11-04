//! Blinks an LED

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_std]
#![no_main]

#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
// extern crate panic_semihosting;
extern crate stm32f103xx_hal as hal;
extern crate panic_itm;
#[macro_use(block)]
extern crate nb;

use hal::prelude::*;
use hal::stm32f103xx;
use hal::timer::Timer;
use rt::ExceptionFrame;

#[no_mangle]
extern "C" {
    fn halPortInit() -> i32;
    // fn halPortFindDevice(id : u16) -> i32;
    fn halPortOpen(id : u16, mode : u16) -> i32;
    fn halPortWrite(handle: i32, value: u16) -> i32;
}

#[entry]
fn main() -> ! {
    
    let mut result = unsafe { halPortInit() };
    // let index = unsafe { halPortFindDevice(0x3200 | 10) };
    let handle_red = unsafe { halPortOpen( 0x3200 | 10, 2) };
    let handle_green = unsafe { halPortOpen( 0x3200 | 11, 2) };

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f103xx::Peripherals::take().unwrap();

    let mut itm = cp.ITM;
    iprintln!(&mut itm.stim[0], "Hello, world!");

    let mut flash = dp.FLASH.constrain();
    // iprintln!(&mut itm.stim[0], "flash {:?}", flash);
    let mut rcc = dp.RCC.constrain();

    // Try a different clock configuration
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // let clocks = rcc.cfgr
    //     .sysclk(64.mhz())
    //     .pclk1(32.mhz())
    //     .freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let mut led_yellow = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut led_blue = gpioc.pc14.into_push_pull_output(&mut gpioc.crh);
    let mut led_red = gpioc.pc15.into_push_pull_output(&mut gpioc.crh);
    // Try a different timer (even SYST)
    let mut timer = Timer::syst(cp.SYST, 1.hz(), clocks);
    loop {
        block!(timer.wait()).unwrap();
        led_blue.set_low();
        led_red.set_high();
        unsafe { halPortWrite(handle_red, 1) };
        unsafe { halPortWrite(handle_green, 0) };

        block!(timer.wait()).unwrap();
        led_red.set_low();
        led_yellow.set_high();

        block!(timer.wait()).unwrap();
        led_yellow.set_low();
        led_blue.set_high();
        unsafe { halPortWrite(handle_red, 0) };
        unsafe { halPortWrite(handle_green, 1) };
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}


// https://github.com/japaric/utest

#[cfg(all(target_arch = "arm",
          not(any(target_env = "gnu", target_env = "musl"))))]
#[macro_use]
extern crate utest_macros;

#[test]
fn assert() {
    assert!(true);
}

// #[test]
// fn assert_failed() {
//     assert!(false, "oh noes");
// }

// #[test]
// fn device_id() {
//     assert_eq!(unsafe { ptr::read_volatile(0xe004_2000 as *const u32) } &
//                ((1 << 12) - 1),
//                0x410);
// }

// #[test]
// fn test_init() {
//     assert_eq!(unsafe { halPortInit() }, 0);
// }


// https://github.com/rust-embedded/book/issues/1#issuecomment-411958378
// https://gist.github.com/japaric/b5e0a7450d968f6ca29c4d238d34a2d0#file-lib-rs-L86
// http://ww1.microchip.com/downloads/en/DeviceDoc/Frequently-Asked-Questions-4.9.3.26.txt

extern crate cty;

// use core::heap::{Alloc, AllocErr, Layout};
// use core::{cmp, ptr};

use cty::c_void;

// Required by malloc
// This implementation is tailored to cortex-m-rt
#[no_mangle]
pub unsafe extern "C" fn _sbrk(nbytes: isize) -> *mut c_void {
    extern "C" {
        static mut __sheap: u8;
        static mut _eheap: u8;
    }

    static mut HEAP: *mut u8 = unsafe { &__sheap as *const u8 as *mut u8 };

    let eheap = &mut _eheap as *mut u8;
    let base = HEAP;
    let new = base.offset(nbytes);
    if new < eheap {
        HEAP = new;

        base as *mut c_void
    } else {
        // OOM
        0 as *mut c_void
    }
}
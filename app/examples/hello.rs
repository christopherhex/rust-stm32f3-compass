//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

use panic_semihosting as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    let xs = [0, 1, 2];
    let i = xs.len() + 1;
    let _y = xs[i]; // out of bounds access

    loop {}
}

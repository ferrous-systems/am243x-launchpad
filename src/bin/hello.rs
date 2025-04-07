//! Semihosting hello-world.

#![no_std]
#![no_main]

// pull in our start-up code
use am243x_launchpad as _;

use semihosting::println;

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[no_mangle]
pub extern "C" fn kmain() -> ! {
    main();
}

/// The main function of our Rust application.
#[export_name = "main"]
fn main() -> ! {
    println!("Hello, this is semihosting on the AM243x Launchpad");
    panic!("I am an example panic");
}

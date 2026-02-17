//! Semihosting hello-world.

#![no_std]
#![no_main]

// pull in our start-up code
use am243x_launchpad as _;

#[aarch32_rt::entry]
fn main() -> ! {
    defmt::println!("This is defmt on the AM243x Launchpad...");
    panic!("I am an example panic");
}

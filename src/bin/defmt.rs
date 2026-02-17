//! Semihosting hello-world.

#![no_std]
#![no_main]

// pull in our start-up code
use am243x_launchpad as _;

use defmt_rtt as _;

// todo: replace with a call to a timer
defmt::timestamp!("{:tus}", 0);

#[aarch32_rt::entry]
fn main() -> ! {
    aarch32_cpu::register::Sctlr::modify(|f| {
        f.with_c(false);
    });
    defmt::println!("Hello, this is defmt!");
    defmt::error!("Hello, this is defmt!");
    defmt::warn!("Hello, this is defmt!");
    defmt::info!("Hello, this is defmt!");
    defmt::debug!("Hello, this is defmt!");
    defmt::trace!("Hello, this is defmt!");
    loop {
        defmt::println!("I am now in a loop...");
        for _ in 0..1_000_000 {
            aarch32_cpu::asm::nop();
        }
    }
}

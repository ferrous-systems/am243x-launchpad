//! SVC (software interrupt) example

#![no_std]
#![no_main]

// pull in our start-up code
use am243x_launchpad as _;

#[aarch32_rt::entry]
fn main() -> ! {
    let x = 1;
    let y = x + 1;
    let z = (y as f64) * 1.5;
    defmt::info!("x = {}, y = {}, z = {}", x, y, z);
    aarch32_cpu::svc!(0xABCDEF);
    defmt::info!("x = {}, y = {}, z = {}", x, y, z);
    panic!("I am an example panic");
}

/// This is our SVC exception handler
#[aarch32_rt::exception(SupervisorCall)]
fn svc_handler(arg: u32, _frame: &aarch32_rt::Frame) -> u32 {
    defmt::println!("In svc_handler, with arg=0x{=u32:06x}", arg);
    if arg == 0xABCDEF {
        // test nested SVC calls
        aarch32_cpu::svc!(0x456789);
    }
    0
}

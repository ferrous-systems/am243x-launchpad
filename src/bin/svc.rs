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
#[no_mangle]
unsafe extern "C" fn _svc_handler(arg: u32) {
    defmt::info!("In _svc_handler, with arg={:#06x}", arg);
    if arg == 0xABCDEF {
        // test nested SVC calls
        aarch32_cpu::svc!(0x456789);
    }
}

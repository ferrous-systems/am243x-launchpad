//! Common code for all examples

#![no_std]

// Need this to bring in the start-up function
use aarch32_rt as _;
use defmt_rtt as _;

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
#[cfg(target_os = "none")]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {:#?}", info);
    semihosting::process::abort();
}


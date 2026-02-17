//! Common code for all examples

#![no_std]

#[cfg(target_arch = "arm")]
use aarch32_cpu::register::{Cpsr, Sctlr, cpsr::ProcessorMode};
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

// Start-up code for multi-core Armv7-R
core::arch::global_asm!(
    r#"
    .section .text.startup
    .align 4
    .arm

    .global _start
    .global core1_released
    .type _start, %function
    _start:
        // Read MPIDR into R0
        mrc     p15, 0, r0, c0, c0, 5
        ands    r0, r0, 0xFF
        bne     core1
    core0:
        ldr     pc, =_default_start
    core1:
    core1_spin:
        wfe
        beq     core1_spin
    core1_released:
        // Allow VFP coprocessor access
        mrc     p15, 0, r0, c1, c0, 2
        orr     r0, r0, #0xF00000
        mcr     p15, 0, r0, c1, c0, 2
        // Enable VFP
        mov     r0, #0x40000000
        vmsr    fpexc, r0
        // Set the VBAR (for EL1) to _vector_table. NB: This isn't required on
        // Armv7-R because that only supports 'low' (default) or 'high'.
        ldr     r0, =_vector_table
        mcr     p15, 0, r0, c12, c0, 0
        // set up our stacks - also switches to SYS mode
        bl      _core1_stack_setup_preallocated
        // Zero all registers before calling kmain2
        mov     r0, 0
        mov     r1, 0
        mov     r2, 0
        mov     r3, 0
        mov     r4, 0
        mov     r5, 0
        mov     r6, 0
        mov     r7, 0
        mov     r8, 0
        mov     r9, 0
        mov     r10, 0
        mov     r11, 0
        mov     r12, 0
        // call our kmain2 for core 1
        bl      kmain2
    .size _start, . - _start
    "#,
);

// Initialise the stack for Core 1 for each mode
#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp2

    // Configure a stack for every mode. Leaves you in sys mode.
    //
    .section .text._core1_stack_setup_preallocated
    .arm
    .global _core1_stack_setup_preallocated
    .type _core1_stack_setup_preallocated, %function
    _core1_stack_setup_preallocated:
        // Save LR from whatever mode we're currently in
        mov     r2, lr
        // (we might not be in the same mode when we return).
        // Set stack pointer and mask interrupts for UND mode (Mode 0x1B)
        msr     cpsr_c, {und_mode}
        ldr	r13, =_core1_und_stack
        // Set stack pointer (right after) and mask interrupts for SVC mode (Mode 0x13)
        msr     cpsr_c, {svc_mode}
        ldr	r13, =_core1_svc_stack
        // Set stack pointer (right after) and mask interrupts for ABT mode (Mode 0x17)
        msr     cpsr_c, {abt_mode}
        ldr	r13, =_core1_abt_stack
        // Set stack pointer (right after) and mask interrupts for IRQ mode (Mode 0x12)
        msr     cpsr_c, {irq_mode}
        ldr	r13, =_core1_irq_stack
        // Set stack pointer (right after) and mask interrupts for FIQ mode (Mode 0x11)
        msr     cpsr_c, {fiq_mode}
        ldr	r13, =_core1_fiq_stack
        // Set stack pointer (right after) and mask interrupts for System mode (Mode 0x1F)
        msr     cpsr_c, {sys_mode}
        ldr	r13, =_core1_sys_stack
        // Clear the Thumb Exception bit because all vector table is written in Arm assembly
        // even on Thumb targets.
        mrc     p15, 0, r1, c1, c0, 0
        bic     r1, #{te_bit}
        mcr     p15, 0, r1, c1, c0, 0
        // return to caller
        bx      r2
    .size _core1_stack_setup_preallocated, . - _core1_stack_setup_preallocated
    "#,
    und_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Und)
            .with_i(true)
            .with_f(true)
            .raw_value()
    },
    svc_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Svc)
            .with_i(true)
            .with_f(true)
            .raw_value()
    },
    abt_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Abt)
            .with_i(true)
            .with_f(true)
            .raw_value()
    },
    fiq_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Fiq)
            .with_i(true)
            .with_f(true)
            .raw_value()
    },
    irq_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Irq)
            .with_i(true)
            .with_f(true)
            .raw_value()
    },
    sys_mode = const {
        Cpsr::new_with_raw_value(0)
            .with_mode(ProcessorMode::Sys)
            .with_i(true)
            .with_f(true)
            .raw_value()
    },
    te_bit = const { Sctlr::new_with_raw_value(0).with_te(true).raw_value() }
);

/// What a second core does when no `kmain2` is supplied.
#[unsafe(no_mangle)]
pub extern "C" fn default_kmain2() {
    loop {
        aarch32_cpu::asm::wfe();
    }
}

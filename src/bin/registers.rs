//! Registers example

#![no_std]
#![no_main]

// pull in our start-up code
use am243x_launchpad as _;

#[aarch32_rt::entry]
fn main() -> ! {
    chip_info();

    for _ in 0..5_000_000 {
        aarch32_cpu::asm::nop();
    }

    test_changing_sctlr();

    for _ in 0..5_000_000 {
        aarch32_cpu::asm::nop();
    }

    mpu_pmsa_v7();

    semihosting::process::exit(0);
}

fn chip_info() {
    defmt::info!("{:?}", aarch32_cpu::register::Midr::read());
    defmt::info!("{:?}", aarch32_cpu::register::Cpsr::read());
    defmt::info!("{:?}", aarch32_cpu::register::Mpidr::read());
}

#[cfg(arm_architecture = "v7-r")]
fn mpu_pmsa_v7() {
    use aarch32_cpu::{
        pmsav7::{CacheablePolicy, Config, MemAttr, Mpu, Region, RegionSize},
        register::Mpuir,
    };

    // How many regions?
    let mpuir = Mpuir::read();
    defmt::info!("PMSA-v7 MPUIR: {:?}", mpuir);

    // Make an MPU driver
    let mut mpu = unsafe { Mpu::new() };

    // Look at the existing config
    for idx in 0..mpu.num_iregions() {
        if let Some(region) = mpu.get_iregion(idx) {
            defmt::info!("IRegion {}: {:?}", idx, defmt::Debug2Format(&region));
        }
    }
    for idx in 0..mpu.num_dregions() {
        if let Some(region) = mpu.get_dregion(idx) {
            defmt::info!("DRegion {}: {:?}", idx, defmt::Debug2Format(&region));
        }
    }

    // Load a config (but don't enable it)
    mpu.configure(&Config {
        background_config: true,
        dregions: &[Region {
            base: 0x0000_0000 as *mut u8,
            size: RegionSize::_64K,
            subregion_mask: 0x00,
            enabled: true,
            no_exec: false,
            mem_attr: MemAttr::Cacheable {
                inner: CacheablePolicy::WriteThroughNoWriteAllocate,
                outer: CacheablePolicy::NonCacheable,
                shareable: true,
            },
        }],
        iregions: &[],
    })
    .unwrap();

    // Look at the new config
    for idx in 0..mpu.num_dregions() {
        if let Some(region) = mpu.get_dregion(idx) {
            defmt::info!("DRegion {}: {:?}", idx, defmt::Debug2Format(&region));
        }
    }
}

fn test_changing_sctlr() {
    defmt::info!(
        "{:?} before setting C, I and Z",
        aarch32_cpu::register::Sctlr::read()
    );
    aarch32_cpu::register::Sctlr::modify(|w| {
        w.set_c(true);
        w.set_i(true);
        w.set_z(true);
    });
    defmt::info!("{:?} after", aarch32_cpu::register::Sctlr::read());
}

#[unsafe(no_mangle)]
extern "C" fn _svc_handler(_svc: u32) {}

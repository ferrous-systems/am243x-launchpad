# Rust Examples for the TI Launchpad™ kit with Sitara™ AM243x™ MCU

This package contains Rust example code for the [TI Launchpad™ kit with Sitara™ AM243x™ MCU](https://www.ti.com/tool/LP-AM243).

This board features:

* Texas Instruments [AM2434 SoC](https://www.ti.com/product/AM2434) (kit also available with other MCUs)
* Two Gigabit Ethernet interfaces
* [512 Mbit QSPI flash](https://www.infineon.com/part/S25HL512TFAMHI010)
* [CAN Transceiver](https://www.ti.com/product/TCAN1044-Q1)
* Five LEDs
* Lots of expansion headers
* On-board XDS110 debug probe

The Texas Instruments AM2434 SoC features:

* 4x Arm Cortex-R5F cores @ 800 MHz
  * Arranged as 2x dual-core clusters
  * Each cluster can run lock-step or independently
  * 32 KiB I-cache and 32 KiB D-cache per core
  * 128 KiB of Tightly Coupled Memory per cluster
* 1x Arm Cortex-M4F @ 400 MHz
  * Dedicated 256 KiB SRAM
* 2 MiB of shared SRAM with ECC
* DDR4 SDRAM Interface
* 2x SD/MMC Interfaces
* Three port Gigabit Ethernet switch
* USB 3.0 SuperSpeed Host
* PCI-Express Gen 2.0 1x
* Two "Gigabit Industrial Communication Subsytems"
  * Each has two Gigabit Ethernet MAC interfaces, three RISC processors, RAM, etc

These examples run on the Cortex-R5F at Core 0, Cluster 0.

## Loading

Currently `probe-rs` has experimental Armv7-R support, in [this PR](https://github.com/probe-rs/probe-rs/pull/3844). You will also need the YAML file in the root directory of this repo, until such time as it gets added to probe-rs.

```console
$ git clone https://github.com/probe-rs/probe-rs
$ cd probe-rs/probe-rs-tools
$ git checkout armv7r-support
$ cargo install --path .
$ cd ~/am243x-launchpad
$ cargo run
cargo run
   Compiling am243x-launchpad v0.1.0 (/Users/jonathan/Documents/ferrous-systems/sitara-launchpad/am243x-launchpad)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `probe-rs run --chip AM2434 --chip-description-path ./AM243.yaml target/armv7r-none-eabihf/debug/hello`
     Finished in 0.76s
This is defmt on the AM243x Launchpad...
[ERROR] PANIC: panicked at "src/bin/hello.rs":12:5 (am243x_launchpad am243x-launchpad/src/lib.rs:16)
Firmware exited with: Application exit (Aborted)
```

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on the latest stable Rust. It *might*
compile with older versions but that may change in any new patch release.

## Licence

* Copyright (c) Ferrous Systems
* Copyright (c) The Rust Embedded Devices Working Group developers

Licensed under either [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at
your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.

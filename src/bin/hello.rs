#![no_main]
#![no_std]

use defmt_test_ignore_bug as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    defmt_test_ignore_bug::exit()
}

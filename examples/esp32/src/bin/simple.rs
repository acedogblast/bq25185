#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use bq25185::driver::Bq25185;
use bq25185::Status;
use defmt::info;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, Output};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println as _;

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // GPIO definitions
    let charger_stat1 = peripherals.GPIO5;
    let charger_stat2 = peripherals.GPIO7;

    // Charger status GPIO
    let config = esp_hal::gpio::InputConfig::default();
    let charger_stat1 = Input::new(charger_stat1, config);
    let charger_stat2 = Input::new(charger_stat2, config);

    let mut charger: Bq25185<Input<'_>, Output<'_>> =
        Bq25185::new(charger_stat1, charger_stat2, None); // Not using the charge enable pin.

    loop {
        match charger.get_status().unwrap() {
            Status::ChargeComplete => info!("ChargeComplete"),
            Status::NonRecoverableFault => info!("NonRecoverableFault"),
            Status::RecoverableFault => info!("RecoverableFault"),
            Status::NormalCharging => info!("NormalCharging"),
        }

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_secs(10) {}
    }
}

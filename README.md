# TI BQ25185 Battery Charger Driver

A basic and simple platform-agnostic driver for the Texas Instruments BQ25185 battery charger IC using [`embedded-hal`] traits.  
Designed for use in `no_std` embedded environments.

---

## Features

- Supports reading the stat1 and stat2 digital pins and returns a `Status` emum.
- Optional charge enable pin managment.
- Compatible with any platform that implements [`embedded-hal`] traits.
- `no_std` support.

---

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
bq25185 = "=0.1.0"
```

### Example on ESP32 using esp_hal
```
#![no_std]
#![no_main]
use bq25185::driver::Bq25185;
use bq25185::Status;
use defmt::{error, info};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, Output};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};

#[panic_handler]
fn panic(_error: &core::panic::PanicInfo) -> ! {
    error!("Panic occured!");
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();

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
```

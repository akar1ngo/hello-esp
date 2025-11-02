#![no_std]
#![no_main]

use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::time::Duration;
use esp_hal::timer::PeriodicTimer;
use esp_hal::timer::timg::TimerGroup;
use panic_rtt_target as _;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);

    let mut led = Output::new(peripherals.GPIO0, Level::High, OutputConfig::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let mut periodic = PeriodicTimer::new(timg0.timer0);

    periodic.start(Duration::from_secs(1)).expect("timer");
    loop {
        led.toggle();
        periodic.wait();
    }
}

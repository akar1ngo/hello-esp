#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);

    let mut led = Output::new(peripherals.GPIO0, Level::High, OutputConfig::default());

    loop {
        led.toggle();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}

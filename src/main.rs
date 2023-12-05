#![no_std]
#![no_main]

use esp32c6_hal::prelude::*;
use esp_backtrace as _;

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    log::info!("Hello, world!");
    loop {}
}

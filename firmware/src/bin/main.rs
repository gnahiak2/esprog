// firmware/src/bin/main.rs
#![no_std]
#![no_main]

use esp_hal::main;

esp_bootloader_esp_idf::esp_app_desc!();

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default();
    let _peripherals = esp_hal::init(config);

    // Insert firmware here
    // v0.1
    // no
    // - SPI
    // - PSRAM
    // - flash access
    // - protocol testing

    loop {
        // waiting for usb commands
    }
}

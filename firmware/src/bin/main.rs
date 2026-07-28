#![no_std]
#![no_main]

use core::panic::PanicInfo;

use esp_hal::{Config, main};

use firmware::{dummy, protocol, usb::Usb};

esp_bootloader_esp_idf::esp_app_desc!();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[main]
fn main() -> ! {
    let config = Config::default();

    let peripherals = esp_hal::init(config);

    let mut usb = Usb::new(peripherals.USB_DEVICE);

    // Initialise dummy memory
    dummy::erase();

    usb.write_line("======================");
    usb.write_line(" ESProg Firmware v0.1");
    usb.write_line(" ESP32-S3");
    usb.write_line(" USB Serial/JTAG");
    usb.write_line("======================");

    let (name, size) = dummy::probe();

    usb.write_str("DEVICE: ");
    usb.write_line(name);

    usb.write_str("MEMORY: ");

    let mut size_buf = [0u8; 16];

    let size_len = number_to_ascii(size, &mut size_buf);

    usb.write(&size_buf[..size_len]);
    usb.write_line(" bytes");

    usb.write_line("READY");

    let mut command_buffer = [0u8; 128];

    let mut index = 0usize;

    loop {
        if let Some(byte) = usb.read_byte() {
            match byte {
                // Enter
                b'\r' | b'\n' => {
                    usb.write_line("");

                    if index > 0 {
                        protocol::handle(&command_buffer[..index], &mut usb);

                        index = 0;
                    }
                }

                // Backspace
                0x08 | 0x7F => {
                    if index > 0 {
                        index -= 1;
                    }
                }

                // Normal character
                _ => {
                    if index < command_buffer.len() {
                        command_buffer[index] = byte;

                        index += 1;

                        // Echo
                        usb.write(&[byte]);
                    }
                }
            }
        }
    }
}

fn number_to_ascii(mut value: usize, out: &mut [u8]) -> usize {
    if value == 0 {
        out[0] = b'0';

        return 1;
    }

    let mut len = 0;

    while value > 0 {
        out[len] = b'0' + (value % 10) as u8;

        value /= 10;

        len += 1;
    }

    out[..len].reverse();

    len
}

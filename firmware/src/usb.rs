// firmware/src/usb.rs

use esp_hal::{Blocking, peripherals::USB_DEVICE, usb_serial_jtag::UsbSerialJtag};

pub struct Usb<'d> {
    serial: UsbSerialJtag<'d, Blocking>,
}

impl<'d> Usb<'d> {
    pub fn new(usb: USB_DEVICE<'d>) -> Self {
        Self {
            serial: UsbSerialJtag::new(usb),
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        let _ = self.serial.write(data);
    }

    pub fn write_str(&mut self, text: &str) {
        self.write(text.as_bytes());
    }

    pub fn write_line(&mut self, text: &str) {
        self.write_str(text);
        self.write(b"\r\n");
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        self.serial.read_byte().ok()
    }

    pub fn flush(&mut self) {
        let _ = self.serial.flush_tx();
    }
}

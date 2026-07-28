// firmware/src/protocol.rs

use crate::{dummy, usb::Usb};

pub fn handle(input: &[u8], usb: &mut Usb) {
    let command = match core::str::from_utf8(input) {
        Ok(v) => v.trim(),
        Err(_) => {
            usb.write_line("ERR UTF8");
            return;
        }
    };

    let mut args = command.split_whitespace();

    match args.next() {
        Some("PING") => {
            usb.write_line("PONG");
        }

        Some("INFO") => {
            let (name, size) = dummy::probe();

            usb.write_str("DEVICE: ");
            usb.write_line(name);

            usb.write_str("SIZE: ");

            let mut buf = [0u8; 16];
            let len = number_to_ascii(size, &mut buf);

            usb.write(&buf[..len]);
            usb.write_line("");
        }

        Some("ERASE") => {
            dummy::erase();
            usb.write_line("OK");
        }

        Some("READ") => {
            let address = match parse_hex(args.next()) {
                Some(v) => v,
                None => {
                    usb.write_line("ERR ADDR");
                    return;
                }
            };

            let length = match parse_hex(args.next()) {
                Some(v) => v,
                None => {
                    usb.write_line("ERR LEN");
                    return;
                }
            };

            if length > 256 {
                usb.write_line("ERR MAX");
                return;
            }

            let mut buffer = [0u8; 256];

            match dummy::read(address, &mut buffer[..length]) {
                Ok(_) => {
                    for byte in &buffer[..length] {
                        write_hex(usb, *byte);
                    }

                    usb.write_line("");
                }

                Err(_) => {
                    usb.write_line("ERR RANGE");
                }
            }
        }

        Some("WRITE") => {
            let address = match parse_hex(args.next()) {
                Some(v) => v,
                None => {
                    usb.write_line("ERR ADDR");
                    return;
                }
            };

            let mut data = [0u8; 256];
            let mut length = 0usize;

            for value in args {
                if length >= data.len() {
                    usb.write_line("ERR MAX");
                    return;
                }

                let byte = match parse_hex(Some(value)) {
                    Some(v) if v <= 0xFF => v as u8,

                    _ => {
                        usb.write_line("ERR BYTE");
                        return;
                    }
                };

                data[length] = byte;
                length += 1;
            }

            match dummy::write(address, &data[..length]) {
                Ok(_) => {
                    usb.write_line("OK");
                }

                Err(_) => {
                    usb.write_line("ERR RANGE");
                }
            }
        }

        Some("FILL") => {
            let value = match parse_hex(args.next()) {
                Some(v) if v <= 0xFF => v as u8,

                _ => {
                    usb.write_line("ERR BYTE");
                    return;
                }
            };

            dummy::fill(value);

            usb.write_line("OK");
        }

        Some("HELP") => {
            usb.write_line("PING");
            usb.write_line("INFO");
            usb.write_line("READ <ADDR> <LEN>");
            usb.write_line("WRITE <ADDR> <DATA>");
            usb.write_line("ERASE");
            usb.write_line("FILL <BYTE>");
        }

        Some(_) => {
            usb.write_line("ERR UNKNOWN");
        }

        None => {}
    }
}

fn parse_hex(value: Option<&str>) -> Option<usize> {
    let value = value?;

    let value = value.strip_prefix("0x").unwrap_or(value);

    usize::from_str_radix(value, 16).ok()
}

fn write_hex(usb: &mut Usb, byte: u8) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";

    let output = [HEX[(byte >> 4) as usize], HEX[(byte & 0x0F) as usize], b' '];

    usb.write(&output);
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

# ESProg

hai!! ESProg is my super cool project that turns an ESP32-S3 into a hardware programmer!!

the goal is to make something that's cheap, open-source, and can read/write SPI flash chips :D

right now it's got working firmware, a desktop CLI, and both of them can talk to each other over the ESP32-S3's native USB!!

---

# cool stuff!

- works over the ESP32-S3's built-in USB port!!
- USB Serial/JTAG
- automatic device detection
- works on macOS
- works on Linux
- firmware is `no_std`
- firmware + CLI are written in Rust
- prebuilt releases
- ESP32-S3 firmware included with releases

---

# firmware

the firmware lives in `firmware/` and is written completely in Rust!!

currently it has:

- USB communication
- command parser
- dummy memory backend
- READ
- WRITE
- ERASE
- FILL
- bounds checking
- modular code
- `no_std`

current layout:

    firmware/
    ├── src/
    │   ├── bin/
    │   │   └── main.rs
    │   ├── dummy.rs
    │   ├── protocol.rs
    │   ├── usb.rs
    │   └── lib.rs
    ├── .cargo/
    │   └── config.toml
    ├── build.rs
    ├── Cargo.toml
    └── Cargo.lock

the firmware targets the ESP32-S3's Xtensa architecture using the ESP Rust toolchain and `esp-hal`.

---

# CLI

the CLI is also written in Rust!!

currently supports:

    PING
    INFO
    READ
    WRITE
    ERASE
    FILL
    HELP

example:

    esprog ping

output:

    PONG

yay!!!

---

# dummy backend

before talking to a real flash chip, i made a fake one!!

this lets me build and test the whole protocol without needing actual SPI flash hardware yet :D

it currently supports:

- read
- write
- erase
- fill
- bounds checking
- 64 KiB virtual memory

for example:

    WRITE 0x0000 AA BB CC DD

    READ 0x0000 04

    AA BB CC DD

the idea is that the command layer doesn't need to care whether it's talking to the dummy backend or a real SPI flash chip later!!

something like:

                  FlashBackend
                     |
              +------+------+
              |             |
              v             v
          DummyFlash     SpiFlash

so once the real flash backend exists, most of the rest of ESProg shouldn't need to change :D

---

# how it works

the basic idea is:

    +-------------+
    |   ESProg    |
    |     CLI     |
    +------+------+
           |
           | USB
           v
    +-------------+
    |  ESP32-S3   |
    |  Firmware   |
    +------+------+
           |
           | SPI
           v
    +-------------+
    |  SPI Flash  |
    |    Chip     |
    +-------------+

the CLI sends commands over USB, the firmware parses them, and the firmware handles the requested operation.

---

# how i made this

ESProg was made with:

- Rust
- ESP32-S3
- `esp-hal`
- native USB
- SPI flash

---

# how to run it!!

## 1. download ESProg

you don't need to build ESProg from source just to use it!!

go to the latest release:

https://github.com/gnahiak2/esprog/releases/latest

download the release files for your platform.

the release includes:

- prebuilt ESProg CLI binaries
- ESP32-S3 firmware
- other files needed for the release

---

## 2. flash the firmware

connect your ESP32-S3 to your computer.

you will need `esptool` to flash the firmware.

find your ESP32-S3's serial port.

on macOS, it will usually look something like:

    /dev/cu.usbmodemXXXX

on Linux, it will usually look something like:

    /dev/ttyACM0

then flash the firmware:

    esptool --chip esp32s3 --port /dev/cu.usbmodemXXXX write-flash 0x0 esprog-firmware.bin

replace `/dev/cu.usbmodemXXXX` with the actual port for your board.

if your firmware release uses a different filename, use the firmware file included in the release instead.

---

## 3. run the CLI

make the CLI executable if needed:

    chmod +x esprog

then run:

    ./esprog ping

you should get:

    PONG

or:

    ./esprog info

which currently returns something like:

    DEVICE: DUMMY
    SIZE: 65536 bytes

---

# project layout

    esprog/
    ├── cli/
    │   └── ...
    │
    ├── firmware/
    │   ├── src/
    │   │   ├── bin/
    │   │   │   └── main.rs
    │   │   ├── dummy.rs
    │   │   ├── protocol.rs
    │   │   ├── usb.rs
    │   │   └── lib.rs
    │   ├── .cargo/
    │   │   └── config.toml
    │   ├── build.rs
    │   ├── Cargo.toml
    │   └── Cargo.lock
    │
    ├── esprog-logo.png
    ├── esprog-logo.svg
    ├── LICENSE
    └── README.md

---

# releases

prebuilt binaries are available from GitHub Releases so you don't need the Rust toolchain just to use ESProg.

latest release:

https://github.com/gnahiak2/esprog/releases/latest

if you want to build or modify ESProg yourself, the full source code is available in this repository.

---

# stuff i still wanna add!!

## firmware

- [x] USB communication
- [x] command protocol
- [x] dummy backend
- [x] read
- [x] write
- [x] erase
- [x] fill
- [ ] real SPI flash support
- [ ] JEDEC chip detection
- [ ] sector erase
- [ ] page programming
- [ ] verify after writing
- [ ] support more flash chips

---

## CLI

- [x] basic commands
- [x] USB communication
- [ ] auto device detection
- [ ] cross-platform support
- [ ] flash `.bin` files
- [ ] dump flash to a file
- [ ] progress bars
- [ ] prettier output
- [ ] better error messages

---

## hardware

- [ ] SPI bridge mode
- [ ] more flash chips
- [ ] automatic chip detection
- [ ] dedicated ESProg hardware

---

# someday maybe™

- [ ] GUI
- [ ] WebUSB
- [ ] universal programmer mode
- [ ] plugin support
- [ ] more programmer protocols

---

# AI DISCLOSURE

AI helped me with:

- Rust compiler errors
- understanding some `esp-hal` stuff
- improving code structure

all of the project design, implementation, testing, debugging, and making everything actually work was done by me!!

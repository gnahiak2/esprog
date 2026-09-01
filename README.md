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

    cargo run -- ping

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

the CLI sends commands over USB, the firmware parses them, and eventually those commands will be translated into operations on a real SPI flash chip!!

---

# how i made this

ESProg was made with:

- Rust
- ESP32-S3
- `esp-hal`
- native USB
- SPI flash (coming soon)

---

# how to run it!!

## 1. clone the repo

    git clone https://github.com/gnahiak2/esprog.git
    cd esprog

---

## 2. install the ESP Rust toolchain

install `espup`:

    cargo install espup

then install the ESP toolchain:

    espup install

then load the environment:

    source ~/export-esp.sh

you may want to add that to your shell startup file so you don't have to run it every time.

---

## 3. build the firmware

go into the firmware directory:

    cd firmware

then build:

    cargo build --release

the firmware uses the ESP32-S3 target automatically through `.cargo/config.toml`.

---

## 4. flash it

with your ESP32-S3 connected, run:

    cargo run --release

this uses the configured `espflash` runner:

    espflash flash --monitor --chip esp32s3

so you don't need to manually specify the generated binary!!

---

## 5. build the CLI

go back to the repository root:

    cd ../cli

then build:

    cargo build --release

---

## 6. try it!!

once the firmware and CLI are running, try:

    cargo run -- ping

you should get:

    PONG

or:

    cargo run -- info

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

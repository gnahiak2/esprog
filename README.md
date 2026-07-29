# ESProg!!! 🦀

hai!! ESProg is my super cool project that turns an ESP32-S3 into a hardware programmer!!the goal is to make something that's cheap, open-source, and can read/write SPI flash chips :D

right now it's got a working firmware, a desktop CLI, and both of them can talk to each other over the ESP32-S3's native USB!

---

# ✨ cool stuff!!## 🔌 USB

- works over the ESP32-S3's built-in USB port!!- USB Serial/JTAG
- automatic device detection
- works on macOS
- works on Linux

---

## 🦀 firmware

written completely in Rust!!currently has:

- USB communication
- command parser
- dummy memory backend
- modular code
- `no_std`

current layout:

```text
firmware/
├── src/
│   ├── bin/
│   │   └── main.rs
│   ├── dummy.rs
│   ├── protocol.rs
│   ├── usb.rs
│   └── lib.rs
└── Cargo.toml
```

---

## 💻 CLI

also written in Rust!!currently supports:

```text
PING
INFO
READ
WRITE
ERASE
FILL
HELP
```

example:

```bash
cargo run -- ping
```

output:

```text
PONG
```

yay!!! :D

---

## 🧪 dummy backend

before talking to a real flash chip, i made a fake one!!it currently supports:

- read
- write
- erase
- fill
- bounds checking
- 64 KiB virtual memory

example:

```text
WRITE 0x0000 AA BB CC DD

READ 0x0000 04

AA BB CC DD
```

---

# 🛠️ how i made this

ESProg was made with:

- Rust 🦀
- ESP32-S3
- esp-hal
- USB Serial/JTAG
- clap
- serialport
- way too much debugging :heavysob:

---

# 🚀 how to run it!!

## 1. clone the repo

```bash
git clone https://github.com/gnahiak2/esprog.git
cd esprog
```

## 2. install the ESP Rust toolchain

```bash
cargo install espup
espup install
```

then load it:

```bash
source ~/export-esp.sh
```

## 3. build the firmware

```bash
cd firmware
cargo build
```

## 4. flash it

```bash
espflash flash target/xtensa-esp32s3-none-elf/debug/esprog-firmware
```

## 5. build the CLI

```bash
cd ../cli
cargo build
```

## 6. try it!!```bash
cargo run -- ping
```

you should get:

```text
PONG
```

or:

```bash
cargo run -- info
```

```text
DEVICE: DUMMY
SIZE: 65536 bytes
```

---

# 🚧 stuff i still wanna add!!## firmware

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

## CLI

- [x] auto device detection
- [x] cross-platform support
- [ ] flash `.bin` files
- [ ] dump flash to a file
- [ ] progress bars
- [ ] prettier output

## hardware

- [ ] SPI bridge mode
- [ ] more flash chips
- [ ] automatic chip detection

## someday maybe™

- [ ] GUI
- [ ] WebUSB
- [ ] universal programmer mode
- [ ] plugin support
- [ ] world domination (optional)

---

# 🤖 AI DISCLOSURE

AI helped me with:

- yelling at Rust compiler errors
- understanding some `esp-hal` stuff
- improving code structure
- writing parts of the documentation

all of the project design, implementation, testing, debugging, and making everything actually work was done by me!!

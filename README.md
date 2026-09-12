# ESProg

ESProg turns an ESP32-S3 into a cheap SPI flash programmer.

```text
Computer
↓ USB
ESP32-S3
↓ SPI
Flash Chip
````

Current Features

Rust firmware
Rust CLI
Native USB
macOS + Linux
`no_std`
Device detection
READ
WRITE
ERASE
FILL
Dummy 64 KiB memory

Next

Real SPI flash support
JEDEC detection
Sector erase
Page programming
Verify
`.bin` flashing
Progress bars

Releases

Prebuilt binaries and firmware:

[https://github.com/gnahiak2/esprog/releases/latest](https://github.com/gnahiak2/esprog/releases/latest)

AI Disclosure

AI helped with compiler errors, `esp-hal`, and code organization.

The project design, implementation, testing, debugging, and getting everything working was done by me.

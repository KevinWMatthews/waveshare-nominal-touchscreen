# Waveshare / Nominal Touchscreen

An ESP32S3-driven Waveshare touchscreen that live streams data into Nominal.

This project is in the early stages of development. If you have questions or if things don't work, please ask!

## Prerequisites

Target hardware:

- Waveshare's [ESP32S3 Touch LCD, 2.8 in](https://www.waveshare.com/product/arduino/displays/lcd-rgb/esp32-s3-touch-lcd-2.8c.htm)

Prerequisites for compiling Waveshare firmware:

- [espup](https://github.com/esp-rs/espup)
- the Xtensa Rust toolchain (see below to install via `espup`)
- [embuild](https://github.com/esp-rs/embuild/tree/master)'s `ldproxy` CLI tool
- [espflash](https://github.com/esp-rs/espflash)

Nominal prerequisites:
- Python 3.14
- [Nominal Connect](https://nominal.io/products/connect)

Additionally, the following tools can be helpful:

- a serial device I/O tool such as [`tio`](https://github.com/tio/tio)
- a USB to UART adapter such as [DSD Tech SH-U09C5](https://www.deshide.com/product-details_SH-U09C5.html)

## Setup

Install `espup`:
```
% cargo install espup
```

Install the Xtensa toolchain for cross-compiling for ESP32S3:
```
% espup install -t esp32s3
```

It should not be necessary to source additional environment variables, as `espup` recommends:
```
To get started, you need to set up some environment variables by running: '. /Users/<YOU>/export-esp.sh'
This step must be done every time you open a new terminal.
```

This can be useful for accessing additional Xtensa toolchain applications (such as `readelf` or `addr2line`).

Install a custom linker tool required by `embuild`, which compiles the ESP-IDF:
```
% cargo install ldproxy
```

Install `espflash` to flash and monitor Espressif devices:
```
% cargo install espflash
```


## Building and Flashing

Firmware for the Waveshare touchscreen can be cross-compiled using a typical Cargo workflow:
```
% cargo build
```

This will:
- download Espressif's ESP-IDF and related tools into the directory `.embuild/`
- cross-compile firmware for `xtensa-esp32s3-espidf`

Note that the download into `.embuild/` extends the time of the first build significantly, and that this directory is
not deleted by `cargo clean`.

Once cross-compilation is complete, firmware can be flashed to a Waveshare touchscreen that is connected to the host via
either of the board's USB connectors, which are labeled:
- USB
- UART

```
% cargo run
```

This will prompt the user to select an appropriate serial device.

To set a persistent mapping for a serial device:
```
# MacOS
% ls /dev/tty.usb*
```

Programming ports for a Waveshare touchscreen have the name `usbmodem`, for example:
```
/dev/tty.usbmodem101
```

The environment variable `ESPFLASH_PORT` will configure `espflash` to automatically select a serial device:
```
% export ESPFLASH_PORT=/dev/tty.usbmodemXXX
% cargo run
```

### Flash with `espflash`

To flash firmware from a specific file, invoke `espflash` directly:
```
% espflash flash --monitor --port /dev/tty.usbmodemXXX <path_to_firmware>
```

### Monitor Serial Console with `espflash`

Espressif's `espflash` tool can attach a console to a device without reflashing:
```
% espflash monitor --port /dev/tty.usbmodemXXX
```

### Monitor Serial Console over UART

A UART serial console can be attached via several connectors on the Waveshare hardware:
- USB-C connector labeled `USB`
- USB-C connector labeled `UART`
- 4-pin SMT connector labeled `UART`
- 12-pin SMT connector (unlabeled)

Use the following UART settings:
- baudrate: 115200
- databits: 8
- parity: none
- stopbits: 1

Note that UART-only connections appear with the name `usbserial`:
```
# MacOS
% ls /dev/tty.usb*
/dev/tty.usbserial-BG01UUCE
```

This is in contrast to programming/UART ports, which are listed as `usbmodem`.

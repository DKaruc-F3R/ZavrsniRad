# Rust for Embedded Systems (NodeMCU-ESP32)

This repository contains `no_std` bare-metal low-level Rust libraries for peripheral control on the NodeMCU-ESP32 (Joy-IT) development board.
The project provides custom modules for IO_MUX, GPIO, the peripheral matrix, and communication protocols (SPI, UART), along with practical examples such as reading temperature data from a BMP280 sensor and controlling LED indicators.

---

## Repository Structure

```
/src
  ├─ io_mux.rs        # IO_MUX registers and functions
  ├─ gpio_mux.rs      # GPIO registers
  ├─ periph.rs        # peripheral signal configuration (IN/OUT SEL)
  ├─ lib.rs           # core GPIO library (Pin<MODE>, set_pull, is_high, …)
  ├─ main.rs          # examples and applications (BMP280, demo)
  └─ protocols/
       ├─ spi.rs
       ├─ uart.rs

```

---

## Project Goals


- Explore **Rust’s applicability in no_std** embedded environments (ESP32).
- Implement low-level **register wrappers** and libraries for safe pin and peripheral signal management.
- Provide practical examples (bit-banging and hardware-mapped) demonstrating usability and limitations of the approach.

---

## Key Features

- `no_std` low-level library for ESP32 
- **Type-state API** (`Pin<Output>`, `Pin<Input>`) ensures compile-time safety for pin configuration  
- `io_mux.rs` — pad-config addresses and bit masks (`MCU_SEL`, `FUN_IE`, `FUN_WPU`, `FUN_WPD`, …) 
- `periph.rs` — signal routing, GPIO driver enable/disable
- Implemented protocols:  
  - **SPI** (bit-bang + `SpiPeriph`)  
  - **UART** (bit-bang + `UartPeriph`)  
- Examples: BMP280 temperature reading and LED control based on temperature

---

## Requirements

Recommended environment: **Linux/WSL2 (Ubuntu)**  
(Windows users should use WSL2)

- `git`, `curl`, `build-essential`, `pkg-config`, `libssl-dev`, `python3`, `pip`
- **Rust** (`rustup` + nightly toolchain za ESP32)
- **espup** (Xtensa GCC toolchain)
- **cargo-generate**
- **cargo-espflash** (flash + serial monitor)

### Quick Installation (Ubuntu / WSL2)

```bash
# 1) prerequisites
sudo apt update
sudo apt install -y build-essential curl git pkg-config libssl-dev python3 python3-pip ca-certificates

# 2) install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# 3) nightly toolchain and ESP32 target
rustup toolchain install nightly
rustup target add xtensa-esp32-none-elf --toolchain nightly

# 4) cargo tools
cargo install cargo-generate
cargo install espup
cargo install cargo-espflash

# 5) Xtensa toolchain
espup install

# (optional) serial port access
sudo usermod -aG dialout $USER
```

---

## Creating a New Project (esp-template)

```bash
cargo generate --git https://github.com/esp-rs/esp-template --name my_esp32_project --branch main
cd moj_esp32_projekt
rustup override set nightly
```

---

## Build & Flash

```bash
# build + flash (release)
cargo espflash /dev/ttyUSB0 --release

# or flash an existing binary
cargo espflash --monitor /dev/ttyUSB0 target/xtensa-esp32-none-elf/release/ime_binarne_datoteke
```

---

## Example: BMP280 Sensor


- Implements SPI communication using a **SpiBitBang** (software SPI).
- Reads the **chip ID (0xD0)**, calibration coefficients, and calculates temperature.
- Based on temperature, lights up LEDs:
  - Blue (cold)  
  - Green (normal)  
  - Red (hot)
 

**Pin mapping (example):**

| Function   | Pin |
|------------|-----|
| MOSI       | 23  |
| MISO       | 19  |
| SCK        | 18  |
| CS         | 5   |
| LED blue   | 32  |
| LED red    | 27  |
| LED green  | 26  |

---

## Design Notes

- `lib.rs` uses **type-state** patterns to detect configuration errors at compile time.
- Niskorazinski pristup registrima (`read_volatile`, `write_volatile`) izoliran je u **unsafe blokove** u malim modulima (`io_mux.rs`, `gpio_mux.rs`, `periph.rs`).
- Low-level register access (`read_volatile`, `write_volatile`) is isolated in small, unsafe blocks inside modules (`io_mux.rs`, `gpio_mux.rs`, `periph.rs`).
- Bit-bang implementations are simple and great for prototyping but depend on CPU timing and are slower than hardware peripherals. 

---

## References

- [esp-template — esp-rs](https://github.com/esp-rs/esp-template)  
- [ESP32 Technical Reference Manual (TRM)](https://www.espressif.com/sites/default/files/documentation/esp32_technical_reference_manual_en.pdf)  
- [ESP32 Datasheet](https://www.espressif.com/sites/default/files/documentation/esp32_datasheet_en.pdf)  
- [Embedded Rust Book](https://docs.rust-embedded.org/book/intro/index.html)  
- [Embedonomicon](https://docs.rust-embedded.org/embedonomicon/)  
- [BMP280 Datasheet](https://cdn-shop.adafruit.com/datasheets/BST-BMP280-DS001-11.pdf)  

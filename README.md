# Primjena jezika Rust za razvoj ugradbenih sustava (NodeMCU-ESP32)

Ovo je repozitorij s `no_std` niskorazinskim Rust bibliotekama za upravljanje periferijom na razvojnoj pločici **NodeMCU-ESP32 (Joy-IT)**.  
Projekt sadrži vlastite module za **IO_MUX**, **GPIO**, perifernu matricu i protokole (**SPI i UART**) te praktične primjere — npr. čitanje temperature s **BMP280** senzora i upravljanje LED indikacijom.

---

## Struktura repozitorija

```
/src
  ├─ io_mux.rs        # IO_MUX registri i funkcije
  ├─ gpio_mux.rs      # GPIO registri
  ├─ periph.rs        # konfiguracija perifernih signala (IN/OUT SEL)
  ├─ lib.rs           # osnovna GPIO biblioteka (Pin<MODE>, set_pull, is_high, …)
  ├─ main.rs          # primjeri i aplikacije (BMP280, demo)
  └─ protocols/
       ├─ spi.rs
       ├─ uart.rs

```

---

## Cilj projekta

- Istražiti primjenu **Rusta u no_std okruženju** za ugradbene sustave (ESP32).  
- Implementirati niskorazinske **wrapper-e registara** i biblioteke za sigurno upravljanje pinovima i perifernim signalima.  
- Pružiti praktične primjere (bit-bang i hardversko mapiranje) koji pokazuju upotrebljivost i ograničenja pristupa.  

---

## Ključne značajke

- `no_std` niskorazinska biblioteka za ESP32  
- **Tip-stanja** (`Pin<Output>`, `Pin<Input>`) osigurava sigurnu konfiguraciju pinova već pri kompilaciji  
- `io_mux.rs` — pad-config adrese i bit-maske (`MCU_SEL`, `FUN_IE`, `FUN_WPU`, `FUN_WPD` …)  
- `periph.rs` — mapiranje IN/OUT signala, omogućavanje/isključivanje GPIO drivera  
- Implementirani protokoli:  
  - **SPI** (bit-bang + `SpiPeriph`)  
  - **UART** (bit-bang + `UartPeriph`)  
- Primjeri: čitanje BMP280 senzora (SPI) i vođenje LED dioda prema temperaturi  

---

## Potrebni alati

Preporučeno okruženje: **Linux/WSL2 (Ubuntu)** ili **macOS**  
(na Windowsu koristiti WSL2)

- `git`, `curl`, `build-essential`, `pkg-config`, `libssl-dev`, `python3`, `pip`
- **Rust** (`rustup` + nightly toolchain za ESP32)
- **espup** (Xtensa GCC toolchain)
- **cargo-generate**
- **cargo-espflash** (flash + serijski monitor)

### Brza instalacija (Ubuntu / WSL2)

```bash
# 1) preduvjeti
sudo apt update
sudo apt install -y build-essential curl git pkg-config libssl-dev python3 python3-pip ca-certificates

# 2) rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# 3) nightly toolchain i target
rustup toolchain install nightly
rustup target add xtensa-esp32-none-elf --toolchain nightly

# 4) cargo alati
cargo install cargo-generate
cargo install espup
cargo install cargo-espflash

# 5) Xtensa toolchain
espup install

# (opcionalno) pristup serijskom uređaju
sudo usermod -aG dialout $USER
```

---

## Kreiranje novog projekta (esp-template)

```bash
cargo generate --git https://github.com/esp-rs/esp-template --name moj_esp32_projekt --branch main
cd moj_esp32_projekt
rustup override set nightly
```

---

## Izgradnja i bljeskanje (flash)

```bash
# build + flash (release)
cargo espflash /dev/ttyUSB0 --release

# ili s već izgrađenom binarnom datotekom
cargo espflash --monitor /dev/ttyUSB0 target/xtensa-esp32-none-elf/release/ime_binarne_datoteke
```

---

## Primjer: BMP280 senzor

- Implementacija SPI komunikacije preko **SpiBitBang** (softverski SPI).  
- Čitanje **ID registra (0xD0)**, učitavanje kalibracijskih koeficijenata i izračun temperature.  
- Na temelju izmjerene vrijednosti pale se LED diode:  
  - plava (hladno)  
  - zelena (normalno)  
  - crvena (vruće)  

**Pin mapping (primjer):**

| Funkcija   | Pin |
|------------|-----|
| MOSI       | 23  |
| MISO       | 19  |
| SCK        | 18  |
| CS         | 5   |
| LED plava  | 32  |
| LED crvena | 27  |
| LED zelena | 26  |

---

## Napomene o dizajnu

- `lib.rs` koristi **tip-stanje** za otkrivanje grešaka konfiguracije već u fazi kompilacije.  
- Niskorazinski pristup registrima (`read_volatile`, `write_volatile`) izoliran je u **unsafe blokove** u malim modulima (`io_mux.rs`, `gpio_mux.rs`, `periph.rs`).  
- Bit-bang implementacije su jednostavne i dobre za prototipiranje, ali ovise o CPU-timingu i sporije su od hardverskih periferija.  

---

## Literatura i poveznice

- [esp-template — esp-rs](https://github.com/esp-rs/esp-template)  
- [ESP32 Technical Reference Manual (TRM)](https://www.espressif.com/sites/default/files/documentation/esp32_technical_reference_manual_en.pdf)  
- [ESP32 Datasheet](https://www.espressif.com/sites/default/files/documentation/esp32_datasheet_en.pdf)  
- [Embedded Rust Book](https://docs.rust-embedded.org/book/intro/index.html)  
- [Embedonomicon](https://docs.rust-embedded.org/embedonomicon/)  
- [BMP280 Datasheet](https://cdn-shop.adafruit.com/datasheets/BST-BMP280-DS001-11.pdf)  

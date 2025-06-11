Primjena programskog jezika Rust za razvoj ugradbenih sustava

Omoguceni svi GPIO driveri, za Input ili Output. 
Glavna logika se nalazi u lib.rs, dok se u main.rs nalazi demonstracija funkcionalnost.
Unutar periph.rs nalazi se integracija gpioa korsiteći periferne module, tj. ne direktnom registar manipulacijom kao što je definirano u lib.rs.

Unutar esp32 technical reference manual (ESP TRM) nalazi se detaljan opis svih registara, te opis funkcionalnosti GPIO i IO muxa
 
![image](https://github.com/user-attachments/assets/22519869-ac1e-4c0a-aa27-bb42a25ad9cf)

![image](https://github.com/user-attachments/assets/e09e15c1-c3e5-434b-96f2-8b4db16d636d)


Funkcionalnost se trenutno prikazuje samo preko UART protokola. 


Izvori:

Template za setup ovog projekta je:
https://github.com/esp-rs/esp-template



https://github.com/esp-rs

https://docs.rust-embedded.org/book/intro/index.html

https://docs.rust-embedded.org/embedonomicon/

https://docs.rust-embedded.org/book/design-patterns/hal/gpio.html

https://www.espressif.com/sites/default/files/documentation/esp32_technical_reference_manual_en.pdf ESP32 TRM (Version 5.3)

https://github.com/espressif/esp-idf/blob/v5.2.5/components/soc/esp32/include/soc/gpio_reg.h

https://www.espressif.com/sites/default/files/documentation/esp32_datasheet_en.pdf

https://docs.rust-embedded.org/book/peripherals/index.html

https://github.com/espressif/rust-esp32-example

https://doc.rust-lang.org/std/

https://google.github.io/comprehensive-rust/bare-metal.html

https://github.com/rust-embedded/embedded-hal

https://docs.esp-rs.org/book/introduction.html
https://github.com/esp-rs/espup

https://doc.rust-lang.org/rustc/platform-support.html

https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/peripherals/uart.html
https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/peripherals/spi_master.html
https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/peripherals/ledc.html
https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/peripherals/i2c.html

https://github.com/espressif/esp-idf/tree/v5.4.1/components/driver](https://github.com/esp-rs/esp-template

https://github.com/esp-rs

https://docs.rust-embedded.org/book/intro/index.html

https://docs.rust-embedded.org/embedonomicon/

https://docs.rust-embedded.org/book/design-patterns/hal/gpio.html

https://www.espressif.com/sites/default/files/documentation/esp32_technical_reference_manual_en.pdf ESP32 TRM (Version 5.3)

https://github.com/espressif/esp-idf/blob/v5.2.5/components/soc/esp32/include/soc/gpio_reg.h

https://www.espressif.com/sites/default/files/documentation/esp32_datasheet_en.pdf

https://docs.rust-embedded.org/book/peripherals/index.html

https://github.com/espressif/rust-esp32-example

https://doc.rust-lang.org/std/

https://google.github.io/comprehensive-rust/bare-metal.html

https://github.com/rust-embedded/embedded-hal

https://docs.esp-rs.org/book/introduction.html
https://github.com/esp-rs/espup

https://doc.rust-lang.org/rustc/platform-support.html

https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/peripherals/uart.html
https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/peripherals/spi_master.html
https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/peripherals/ledc.html
https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/peripherals/i2c.html


https://github.com/espressif/esp-idf/tree/v5.4.1/components/driver
https://github.com/esp-rs/no_std-training

https://www.ti.com/lit/an/slva704/slva704.pdf?ts=1749492423884&ref_url=https%253A%252F%252Fwww.google.com%252F)

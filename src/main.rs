#![allow(unused_imports, dead_code)]

use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::log::EspLogger;
use zavrad::protocols::spi::SpiBitBang;
use zavrad::*;

fn main() {

    esp_idf_svc::sys::link_patches();
    // Postavlja ispis na serijski monitor
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("BMP280 SPI Test - citanje temperature");

    // Definicja pinova
    const MOSI_PIN: u8 = 23;
    const MISO_PIN: u8 = 19;
    const SCK_PIN: u8 = 18;
    const CS_PIN: u8 = 5;

    // Inicijalizacija programskog SPI mastera s frekvencijom 1 MHz
    let mut spi = SpiBitBang::new(MOSI_PIN, MISO_PIN, SCK_PIN, CS_PIN, 1_000_000);

    // Citanje ID registra (0x0D)
    let mut id_buff = [0u8;1];
    spi.read_regs(0xD0, &mut id_buff);
    let id = id_buff[0];
    log::info!("BMP280 ID = 0x{:02X} (ocek. 0x58)", id);

    // Citanje kalibracijskih podataka (0x88... 0x9F)
    let mut calib = [0u8; 24];
    spi.read_regs(0x88, &mut calib);

    // Parsiranje konstanti prema formatu iz BMP280 datasheeta
    let dig_t1 = u16::from_le_bytes([calib[0], calib[1]]);
    let dig_t2 = i16::from_le_bytes([calib[2], calib[3]]);
    let dig_t3 = i16::from_le_bytes([calib[4], calib[5]]);

    log::info!("Calib: T1={} T2={} T3={}", dig_t1, dig_t2, dig_t3);

    // Parsiranje kalibracija T (ctrl_meas 0xF4 i config 0xF5)
    spi.write_reg(0xF4, 0x27); // osrs_t=1, osrs_p=1, mode=normal
    spi.write_reg(0xF5, 0xA0); // standby=1000ms, filter off
    FreeRtos::delay_ms(100);

    // Inicjalizacija LED izlaznih pinova
    let led_blue = Pin::<Output>::new(32);
    let led_red = Pin::<Output>::new(27);
    let led_green = Pin::<Output>::new(26);

    loop {
        // Citanje sirove 20 bitne temperature
        let mut raw_t = [0u8; 3];
        spi.read_regs(0xFA, &mut raw_t);
        let adc_t: i32 = ((raw_t[0] as i32) << 12)
                       | ((raw_t[1] as i32) << 4)
                       | ((raw_t[2] as i32) >> 4);

        // Blok prema BMP280 datasheetu (Temperature compensation)
        //Izracun var1, var2, t_fine i temperature u stotinkama stupnja C
        let var1: i32 = (((adc_t >> 3) - ((dig_t1 as i32) << 1)) * (dig_t2 as i32)) >> 11;
        let var2: i32 = (((((adc_t >> 4) - (dig_t1 as i32)) * ((adc_t >> 4) - (dig_t1 as i32))) >> 12) * (dig_t3 as i32)) >> 14;
        let t_fine: i32 = var1 + var2;
        let temp: i32 = (t_fine * 5 + 128) >> 8; // Stotinke stupnja C prema formuli
        let temp_c: f32 = temp as f32 / 100.0;

        // Logika za LED diode
        if temp_c <= 27.0 {
            // Hladno -> plava
            led_blue.set_high();
            led_green.set_low();
            led_red.set_low();
        }  else if temp_c >= 32.0 {
            // Vruće -> crvena
            led_blue.set_low();
            led_green.set_low();
            led_red.set_high();
        } else {
            // Normalno -> zelena
            led_blue.set_low();
            led_green.set_high();
            led_red.set_low();
        }

        log::info!("ADC_T = {}, Temp = {:.2} °C", adc_t, temp_c);

        FreeRtos::delay_ms(1000);
    }

}
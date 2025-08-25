#![allow(unused_imports, dead_code)]
use std::thread;

use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::log::EspLogger;
use zavrad::protocols::spi::SpiBitBang;
use zavrad::protocols::uart::UartBitBang;
use zavrad::*;

//
// 
fn main() {
  
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("BMP280 SPI Test - citanje temperature");

    const MOSI_PIN: u8 = 23;
    const MISO_PIN: u8 = 19;
    const SCK_PIN: u8 = 18;
    const CS_PIN: u8 = 5;

    let mut spi = SpiBitBang::new(MOSI_PIN, MISO_PIN, SCK_PIN, CS_PIN, 1_000_000);


    fn read1(spi: &mut SpiBitBang, reg: u8) -> u8 {
        let mut b = [0u8; 1];
        spi.read_regs(reg, &mut b); 
        b[0]
    }
    fn write1(spi: &mut SpiBitBang, reg: u8, val: u8) {
        spi.write_reg(reg, val);  
    }

     let id = read1(&mut spi, 0xD0);
    log::info!("BMP280 ID = 0x{:02X} (ocek. 0x58)", id);

    let mut calib = [0u8; 24];
    spi.read_regs(0x88, &mut calib);

    let dig_t1 = u16::from_le_bytes([calib[0], calib[1]]);
    let dig_t2 = i16::from_le_bytes([calib[2], calib[3]]);
    let dig_t3 = i16::from_le_bytes([calib[4], calib[5]]);

    log::info!("Calib: T1={} T2={} T3={}", dig_t1, dig_t2, dig_t3);

    write1(&mut spi, 0xF4, 0x27); // osrs_t=1, osrs_p=1, mode=normal
    write1(&mut spi, 0xF5, 0xA0); // standby=1000ms, filter off
    FreeRtos::delay_ms(100);


    
    let led_blue = Pin::<Output>::new(32);
    let led_red = Pin::<Output>::new(27);
    let led_green = Pin::<Output>::new(26);

    loop {
        let mut raw_t = [0u8; 3];
        spi.read_regs(0xFA, &mut raw_t);
        let adc_t: i32 = ((raw_t[0] as i32) << 12)
                       | ((raw_t[1] as i32) << 4)
                       | ((raw_t[2] as i32) >> 4);

        let var1: i32 = (((adc_t >> 3) - ((dig_t1 as i32) << 1)) * (dig_t2 as i32)) >> 11;
        let var2: i32 = (((((adc_t >> 4) - (dig_t1 as i32)) * ((adc_t >> 4) - (dig_t1 as i32))) >> 12) * (dig_t3 as i32)) >> 14;
        let t_fine: i32 = var1 + var2;
        let temp: i32 = (t_fine * 5 + 128) >> 8; // stotinke °C
        let temp_c: f32 = temp as f32 / 100.0;



        //  led_blue.set_pull(Pull::Down);
        //  led_green.set_pull(Pull::Down);
        //  led_red.set_pull(Pull::Down);

    if temp_c <= 29.0 {
            // Hladno → plava
            led_blue.set_high();
            led_green.set_low();
            led_red.set_low();
     }  else if temp_c >= 32.0 {
            // Vruće → crvena
            led_blue.set_low();
            led_green.set_low();
            led_red.set_high();
     } else {
            // Normalno → zelena
            led_blue.set_low();
            led_green.set_high();
            led_red.set_low();
        }

        log::info!("ADC_T = {}, Temp = {:.2} °C", adc_t, temp_c);

        FreeRtos::delay_ms(1000);
    }

}


   // log::info!("LED Test...");

    // let led = Pin::<Output>::new(5);
    // led.set_high();
    // FreeRtos::delay_ms(2000);
    // led.set_low();
 
    // log::info!("UART bitbang test...");

    // let mut uart = UartBitBang::new(27, 14, 115_200);

    // let message = b"ZaVrSnI RaD";
    // let mut received = [0u8; 11];

    // for i in 0..message.len() {
    //     // pošalji i istovremeno pročitaj
    //     received[i] = uart.transfer_byte(message[i]);
    //     // yield za watchdog
    //     FreeRtos::delay_ms(0);
    // }

    // // ostatak: logiranje
    // if let Ok(text) = core::str::from_utf8(&received) {
    //     log::info!("Primljeno s transfer: {}", text);
    // } else {
    //     log::error!("Nevaljani UTF-8 u primljenom nizu");
    // }




    // loop {
    //     FreeRtos::delay_ms(1000);
    // }

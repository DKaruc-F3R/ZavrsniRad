
#![allow(unused_imports,dead_code)]
use std::thread;

use esp_idf_svc::hal::delay::{FreeRtos};
use zavrad::protocols::uart::UartBitBang;
use zavrad::*;
use esp_idf_svc::log::EspLogger;
use crate::protocols::i2c::{I2cMasterBitBang, I2cSlaveBitBang};


// 


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("LED Test...");

    let led = Pin::<Output>::new(5);
    led.set_high();
    FreeRtos::delay_ms(2000);
    led.set_low();


    log::info!("UART bitbang test...");

    let mut uart = UartBitBang::new(27, 14, 115_200);

    let message = b"ZaVrSnI RaD";
    let mut received = [0u8; 11];

    for i in 0..message.len() {
        // pošalji i istovremeno pročitaj
        received[i] = uart.transfer_byte(message[i]);
        // yield za watchdog
        FreeRtos::delay_ms(0);
    }

    // ostatak: logiranje
    if let Ok(text) = core::str::from_utf8(&received) {
        log::info!("Primljeno s transfer: {}", text);
    } else {
        log::error!("Nevaljani UTF-8 u primljenom nizu");
    }

    //jos samo i2c

    loop {
        FreeRtos::delay_ms(1000);
    }


 
}

/*    

const SDA_PIN_M: u8 = 22;
// const SCL_PIN_M: u8 = 23;
// const I2C_FREQ_HZ: u32 = 100_000;
// const MESSAGE: &[u8] = b"Zavrsni Rad FER 2025";
// const SLAVE_ADDRS: [u8; 3] = [0x10, 0x11, 0x12];
// const SDA_PIN_S : [u8; 3] = [18,19,21];
// const SCL_PIN_S : [u8; 3] = [25,33,32];



        log::info!("I2C bitbang test...");

    let mut handles = Vec::new();
    for (i, &addr) in SLAVE_ADDRS.iter().enumerate() {
        let sda = SDA_PIN_S[i];
        let scl = SCL_PIN_S[i];
        handles.push(thread::spawn(move || {
            let mut slave = I2cSlaveBitBang::new(addr, sda, scl, I2C_FREQ_HZ);
            println!("Created slave {} on {:X} ",i,addr);
            loop {
                if slave.check_address() {
                    println!("Slave {} on {:X} checked",i,addr);
                    let mut buf = [0u8; MESSAGE.len()];
                    for idx in 0..buf.len() {
                        buf[idx] = slave.read_data();
                    }
                    if let Ok(s) = core::str::from_utf8(&buf) {
                        println!("Slave 0x{:X} primio: {}", addr, s);
                    }
                }
                FreeRtos::delay_ms(1);
            }
        }));
    }

    // Spawn master dretvu
    let master = thread::spawn(|| {
        let mut master = I2cMasterBitBang::new(SDA_PIN_M, SCL_PIN_M, I2C_FREQ_HZ);
        // kratka pauza da se slave dretve postave
        FreeRtos::delay_ms(1);

        for &addr in &SLAVE_ADDRS {
            master.start();
            let addr_byte = (addr << 1) | 0;
            if !master.write_byte(addr_byte) {
                println!("Master: 0x{:X} nije ACK", addr);
            }
            for &b in MESSAGE {
                let _ = master.write_byte(b);
            }
            master.stop();
        }
        println!("Master: završio slanje");
    });

    // Čekaj da sve dretve završe (u ovom slučaju one rade beskonačno, pa će se program zablokirati ovdje)
    drop(handles);

    let _ = master.join(); */
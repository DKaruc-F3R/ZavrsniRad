#![allow(dead_code)]


use crate::periph::{peripheral_input_signal, peripheral_output_signal};
use crate::{Pin, Pull};
use core::arch::asm;
use core::sync::atomic::{compiler_fence, Ordering};

pub struct Master;
pub struct Slave;

pub struct I2cMasterBitBang {
    sda_pin: u8,
    scl_pin: u8,
    sda: Pin<crate::Output>, // data pin
    scl: Pin<crate::Output>, // clock pin
    bit_delay_cycles: u32,
}

impl I2cMasterBitBang {
    pub fn new(sda_pin: u8, scl_pin: u8, freq: u32) -> Self {
        let cycles = 240_000_000 / freq;
        let sda = Pin::<crate::Output>::new(sda_pin);
        let scl = Pin::<crate::Output>::new(scl_pin);
        sda.set_high();
        scl.set_high();
        I2cMasterBitBang {
            sda_pin,
            scl_pin,
            sda,
            scl,
            bit_delay_cycles: cycles,
        }
    }

    fn bit_delay(&self) {
        for _ in 0..self.bit_delay_cycles {
            unsafe { asm!("nop") };
        }
        compiler_fence(Ordering::SeqCst);
    }

    // i2c start uvijet
    pub fn start(&mut self) {
        self.sda.set_high();
        self.bit_delay();

        self.scl.set_high();
        self.bit_delay();

        self.sda.set_low();
        self.bit_delay();

        self.scl.set_low();
        self.bit_delay();
    }

    // i2c stop uvijer
    pub fn stop(&mut self) {
        self.sda.set_low();
        self.bit_delay();
        self.scl.set_high();
        self.bit_delay();
        self.sda.set_high();
        self.bit_delay();
    }

    pub fn write_byte(&mut self, byte: u8) -> bool {
        for i in 0..8 {
            if (byte & (1 << (7 - i))) != 0 {
                self.sda.set_high();
            } else {
                self.sda.set_low();
            }
            self.bit_delay();
            self.scl.set_high();
            self.bit_delay();
            self.scl.set_low();
            self.bit_delay();
        }

        let sda_in = Pin::<crate::Input>::new(self.sda_pin);
        sda_in.set_pull(Pull::Up);
        self.bit_delay();
        self.scl.set_high();
        self.bit_delay();
        let ack = !sda_in.is_high();
        self.scl.set_low();
        self.bit_delay();
        ack
    }

    pub fn read_byte(&mut self, ack: bool) -> u8 {
        let mut byte = 0u8;
        // Čitanje podataka: SDA kao ulaz
        let sda_in = Pin::<crate::Input>::new(self.sda_pin);
        sda_in.set_pull(Pull::Up);
        for i in 0..8 {
            self.scl.set_high();
            self.bit_delay();
            if sda_in.is_high() {
                byte |= 1 << (7 - i);
            }
            self.scl.set_low();
            self.bit_delay();
        }
        if ack {
            self.sda.set_low(); // ACK = 0
        } else {
            self.sda.set_high(); // NACK = 1
        }
        self.bit_delay();
        self.scl.set_high();
        self.bit_delay();
        self.scl.set_low();
        self.bit_delay();
        byte
    }
}

pub struct I2cSlaveBitBang {
    address: u8,
    sda_pin: u8,
    scl_pin: u8,
    sda: Pin<crate::Output>, // ack pin
    scl: Pin<crate::Input>,  // clock pin
    bit_delay_cycles: u32,
}

impl I2cSlaveBitBang {
    pub fn new(address: u8, sda_pin: u8, scl_pin: u8, freq: u32) -> Self {
        let cycle = 240_000_000 / freq;

        //za slanje ack signala
        let sda = Pin::<crate::Output>::new(sda_pin);
        sda.set_high();
        let scl = Pin::<crate::Input>::new(scl_pin);
        I2cSlaveBitBang {
            address,
            sda_pin,
            scl_pin,
            sda,
            scl,
            bit_delay_cycles: cycle,
        }
    }

    fn bit_delay(&self) {
        for _ in 0..self.bit_delay_cycles {
            unsafe { asm!("nop") };
        }
        compiler_fence(Ordering::SeqCst);
    }
    pub fn check_address(&mut self) -> bool {
        // Čekaj START uvjet: SDA ide HIGH->LOW dok je SCL HIGH
        while !(Pin::<crate::Input>::new(self.sda_pin).is_high() && self.scl.is_high()) {}
        self.bit_delay();
        // Čitaj adresni bajt
        let mut recv = 0u8;
        for i in 0..8 {
            while !self.scl.is_high() {}
            self.bit_delay();
            if Pin::<crate::Input>::new(self.sda_pin).is_high() {
                recv |= 1 << (7 - i);
            }
            while self.scl.is_high() {}
        }
        let addr = recv >> 1;
        let is_write = (recv & 1) == 0;
         log::info!(
        "I2C slave (0x{:02X}): primljena adresa 0x{:02X}, očekivana 0x{:02X}, write={}",
        self.address,
        addr,
        self.address,
        is_write
    );
        if is_write && addr == self.address {
            self.sda.set_low();
            self.bit_delay();
            while !self.scl.is_high() {}
            self.bit_delay();
            while self.scl.is_high() {}
            self.sda.set_high();
            true
        } else {
            false
        }
    }

    /// Čita jedan podatkovni bajt nakon što je adresa potvrđena
    pub fn read_data(&mut self) -> u8 {
        let mut data = 0u8;
        for i in 0..8 {
            while !self.scl.is_high() {}
            self.bit_delay();
            if Pin::<crate::Input>::new(self.sda_pin).is_high() {
                data |= 1 << (7 - i);
            }
            while self.scl.is_high() {}
        }
        data
    }
}

pub struct I2cPeriph;

impl I2cPeriph {
    pub fn init(sda_pin: u8, scl_pin: u8, per_sda_sig: u8, per_scl_sig: u8, pull: Pull) {
        const I2C_SCL_SIGNAL: u8 = 29;
        const I2C_SDA_SIGNAL: u8 = 30;

        // SDA kao open-drain periferni izlaz
        peripheral_output_signal(sda_pin, per_sda_sig, true, 0);

        // SCL kao push-pull periferni izlaz
        peripheral_output_signal(scl_pin, per_scl_sig, false, 0);

        // Hardversko čitanje SDA linije
        peripheral_input_signal(sda_pin, per_sda_sig, pull);
    }
}

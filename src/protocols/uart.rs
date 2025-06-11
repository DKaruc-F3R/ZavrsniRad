#![allow(dead_code)]

use crate::periph::{peripheral_input_signal, peripheral_output_signal};
use crate::{Pin, Pull};
use core::arch::asm;
use core::sync::atomic::{compiler_fence, Ordering};

pub struct UartBitBang {
    tx: Pin<crate::Output>, // transiever signal
    rx: Pin<crate::Input>,  // receiver signal
    bit_delay_cycles: u32,
}

impl UartBitBang {
    pub fn new(tx_pin: u8, rx_pin: u8, baud_rate: u32) -> Self {
        let cycles = 240_000_000 / baud_rate;
        let tx = Pin::<crate::Output>::new(tx_pin);
        tx.set_high();
        let rx = Pin::<crate::Input>::new(rx_pin);
        rx.set_pull(crate::Pull::Up);
        UartBitBang{tx, rx, bit_delay_cycles: cycles}
    }

    #[inline(always)]
    fn bit_delay(&self) {
        // izvrsava NOP instrukcije za odredeni broj ciklusas
        for _ in 0..self.bit_delay_cycles {
            unsafe { asm!("nop") };
        }
        // osigurava ispravan redoslijed operacija
        compiler_fence(Ordering::SeqCst);
    }

       pub fn write_byte(&mut self, byte: u8) {
        // start bit
        self.tx.set_low();
        self.bit_delay();

        // podatkovni bitovi
        for i in 0..8 {
            if (byte >> i) & 1 == 1 {
                self.tx.set_high();
            } else {
                self.tx.set_low();
            }
            self.bit_delay();
        }

        // stop bit
        self.tx.set_high();
        self.bit_delay();
    }


    pub fn read_byte(&mut self) -> u8{
        
        // čekaj start bit (LOW)
        while self.rx.is_high(){
        }

        // sredina prvog podatkovnog bita
        for _ in 0..(self.bit_delay_cycles / 2 ){
            unsafe {asm!("nop")}
        }

        let mut byte = 0;
        for i in 0..8{
            self.bit_delay();
            if self.rx.is_high(){
                byte |= 1 << i
            }
        }

        // preskoči stop bit
        self.bit_delay();
        byte
    }

    pub fn transfer_byte(&mut self, tx_byte: u8) -> u8 {
        let mut rx_byte = 0u8;

        // start bitovi
        self.tx.set_low();
        self.bit_delay();

        for i in 0..8 {
            if (tx_byte >> i) & 1 == 1 {
                self.tx.set_high();
            } else {
                self.tx.set_low();
            }

            for _ in 0..(self.bit_delay_cycles / 2) {
                unsafe { core::arch::asm!("nop") };
            }

            // ocitaj rx tokom sredine
            if self.rx.is_high() {
                rx_byte |= 1 << i;
            }

            self.bit_delay();
        }

        // stop bit
        self.tx.set_high();
        self.bit_delay();

        rx_byte
    }

}


pub struct UartPeriph;

impl UartPeriph{
    
     pub fn init(tx_pin: u8, rx_pin: u8, pull: Pull) {
        const U0RXD_SIGNAL: u8 = 14;
        // konfiguriraj TX pin kao izlazni peripherni signal UART0
        peripheral_output_signal(tx_pin, U0RXD_SIGNAL,  false, 0);
        // konfiguriraj RX pin kao ulazni peripherni signal UART0
        peripheral_input_signal(rx_pin, U0RXD_SIGNAL, pull);
    }

}
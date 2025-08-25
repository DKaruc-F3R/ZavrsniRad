#![allow(dead_code)]

use core::arch::asm;
use core::sync::atomic::{compiler_fence, Ordering};
use crate::{Pin, Pull};
use crate::periph::{peripheral_input_signal, peripheral_output_signal};

// SPI Bitbang Master implementcaija
pub struct SpiBitBang {
    mosi: Pin<crate::Output>,
    miso: Pin<crate::Input>,
    clk: Pin<crate::Output>,
    cs: Pin<crate::Output>,
    bit_delay_cycles: u32,
}

impl SpiBitBang {

    pub fn new(
        mosi_pin: u8,
        miso_pin: u8,
        clk_pin: u8,
        cs_pin: u8,
        freq: u32,
    ) -> Self {
        let cycles = 240_000_000 / freq / 2; // pola ciklusa za svaki brid
        let mosi = Pin::<crate::Output>::new(mosi_pin);
        let miso  =  Pin::<crate::Input>::new(miso_pin);
        let clk = Pin::<crate::Output>::new(clk_pin);
        let cs = Pin::<crate::Output>::new(cs_pin);

        cs.set_high();
        mosi.set_low();
        clk.set_low();

        SpiBitBang { mosi, miso, clk, cs, bit_delay_cycles: cycles }

    }

    #[inline(always)]
    fn bit_delay(&self) {
        for _ in 0..self.bit_delay_cycles {
            unsafe { asm!("nop") };
        }
        compiler_fence(Ordering::SeqCst);
    }

     pub fn transfer_bytes(&mut self, write: &[u8], read: &mut [u8]) {
        assert_eq!(write.len(), read.len());
        self.cs.set_low();
        self.bit_delay();

        for (b, r) in write.iter().zip(read.iter_mut()) {
            let mut tx = *b;
            let mut rx = 0u8;
            for i in (0..8).rev() {
                // Postavi MOSI na MSB
                if (tx & 0x80) != 0 { self.mosi.set_high(); } else { self.mosi.set_low(); }
                tx <<= 1;

                self.bit_delay();

                self.clk.set_high();

                self.bit_delay();

                if self.miso.is_high() {
                    rx |= 1 << i;
                }

                // Padajući brid, priprema za sljedeći bit
                self.clk.set_low();
            }
            *r = rx;
        }

        self.bit_delay();
        self.cs.set_high();
        self.bit_delay();
    }

     pub fn write_reg(&mut self, reg: u8, val: u8) {
        let tx = [reg & 0x7F, val];
        let mut rx = [0u8; 2];
        self.transfer_bytes(&tx, &mut rx);
    }

    /// Pročitaj N bajtova počevši od registra (addr s MSB=1 za read).
    pub fn read_regs(&mut self, start_reg: u8, buf: &mut [u8]) {
        let mut tx = [0u8; 1 + 32]; 
        let mut rx = [0u8; 1 + 32];
        assert!(buf.len() <= 32);

        tx[0] = start_reg | 0x80; 
        for i in 0..buf.len() { tx[1 + i] = 0x00; }

        self.transfer_bytes(&tx[..(1 + buf.len())], &mut rx[..(1 + buf.len())]);
        buf.copy_from_slice(&rx[1..(1 + buf.len())]);
    }

}



pub struct SpiPeriph;

impl SpiPeriph {

    pub fn init(
        mosi_pin: u8,
        miso_pin: u8,
        clk_pin:  u8,
        cs_pin:   u8,
        pull:     Pull,
    ) {
        const SPICLK_SIG: u8 = 0;
        const SPIQ_SIG:   u8 = 1;
        const SPID_SIG:   u8 = 2;
        const SPICS0_SIG: u8 = 5;

        // MOSI (SPIQ) push-pull
        peripheral_output_signal(mosi_pin, SPIQ_SIG,    false, 0);
        // CLK (SPICLK) push-pull
        peripheral_output_signal(clk_pin,  SPICLK_SIG,  false, 0);
        // CS (SPICS0) push-pull
        peripheral_output_signal(cs_pin,   SPICS0_SIG,  false,  0);
        // MISO (SPID) kao ulaz
        peripheral_input_signal(miso_pin, SPID_SIG,    pull);
    }
}
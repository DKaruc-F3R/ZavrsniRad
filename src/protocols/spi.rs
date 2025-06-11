#![allow(dead_code)]

use core::arch::asm;
use core::sync::atomic::{compiler_fence, Ordering};
use crate::{Pin, Pull};
use crate::periph::{peripheral_input_signal, peripheral_output_signal};

// SPI Bitbang Master implementcaija
pub struct SpiBitBang {
    mosi: Pin<crate::Output>,
    miso_pin: u8,
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
        let cycles = 240_000_000 / freq / 2; // pola ciklusa za svaki edge
        let mosi = Pin::<crate::Output>::new(mosi_pin);
        let clk = Pin::<crate::Output>::new(clk_pin);
        let cs = Pin::<crate::Output>::new(cs_pin);
        cs.set_high();
        mosi.set_low();
        clk.set_low();
        SpiBitBang { mosi, miso_pin, clk, cs, bit_delay_cycles: cycles }
    }

    #[inline(always)]
    fn bit_delay(&self) {
        for _ in 0..self.bit_delay_cycles {
            unsafe { asm!("nop") };
        }
        compiler_fence(Ordering::SeqCst);
    }

    pub fn transfer(&mut self, write: &[u8], read: &mut [u8]) {
        // aktiviraj CS
        self.cs.set_low();
        for (b, r) in write.iter().zip(read.iter_mut()) {
            let mut byte = *b;
            let mut recv = 0u8;
            for i in (0..8).rev() {
                // postavi MOSI na MSB
                if (byte & 0x80) != 0 {
                    self.mosi.set_high();
                } else {
                    self.mosi.set_low();
                }
                byte <<= 1;

                // pred-edge delay
                self.bit_delay();
                // podigni CLK
                self.clk.set_high();
                self.bit_delay();
                // čitaj MISO
                let in_pin = Pin::<crate::Input>::new(self.miso_pin);
                if in_pin.is_high() {
                    recv |= 1 << i;
                }
                // spusti CLK
                self.clk.set_low();
            }
            *r = recv;
        }
        // deaktiviraj CS
        self.cs.set_high();
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
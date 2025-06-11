
#![allow(dead_code)]

use core::arch::asm;
use core::sync::atomic::{compiler_fence, Ordering};
use crate::Pin;
use crate::periph::{peripheral_output_signal};

/// Softverski PWM (bit-bang) implementacija
pub struct PwmBitBang {
    pin: u8,
    period_cycles: u32,
    high_cycles: u32,
}

impl PwmBitBang {
    const CPU_FREQ_HZ: u32 = 240_000_000;

    pub fn new(pin: u8, freq: u32, duty_fraction: f32) -> Self {
        let period = Self::CPU_FREQ_HZ / freq;
        let high = (period as f32 * duty_fraction) as u32;
        let gpio = Pin::<crate::Output>::new(pin);
        // idle low
        gpio.set_low();
        PwmBitBang { pin, period_cycles: period, high_cycles: high }
    }

    #[inline(always)]
    fn cycle_delay(cycles: u32) {
        for _ in 0..cycles {
            unsafe { asm!("nop"); }
        }
        compiler_fence(Ordering::SeqCst);
    }

    // Generira jedan PWM period (blockirajuće)
    pub fn tick(&self) {
        let gpio = Pin::<crate::Output>::new(self.pin);
        // high phase
        gpio.set_high();
        Self::cycle_delay(self.high_cycles);
        // low phase
        gpio.set_low();
        Self::cycle_delay(self.period_cycles - self.high_cycles);
    }
}



pub struct PwmPeriph;

impl PwmPeriph {

    pub fn init(pin: u8) {
        const LEDC_CH0_OUT_SIG: u8 = 32; 
        // mapiraj pin na LEDC signal
        peripheral_output_signal(pin, LEDC_CH0_OUT_SIG,  false, 0);
        // Nakon toga, korisnik mora konfigurirati LEDC_CONF registre 
        //Mora se konfigurirati LEDC_CONFIG
        // (npr. LEDC_TIMER0_CONF, LEDC_CHANNEL0_HPOINT, LEDC_CHANNEL0_DUTY)
    }
}

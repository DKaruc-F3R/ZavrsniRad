#![no_std]
#![allow(dead_code)]
#![feature(asm_experimental_arch)]

pub mod gpio_mux;
pub mod io_mux;
pub mod periph;
pub mod protocols;

use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};

pub struct Input;
pub struct Output;

pub enum Pull {
    None,
    Up,
    Down,
}

pub struct Pin<MODE> {
    pin: u8,
    _mode: PhantomData<MODE>,
}

impl Pin<Input> {
    pub fn new(pin: u8) -> Self {
        assert!(pin <= 39, "Pin izvan domene 0-39");
        let p = Pin {
            pin,
            _mode: PhantomData,
        };
        p.config_input();
        p
    }

    fn config_input(&self) {
        if self.pin < 32 {
            unsafe { write_volatile(gpio_mux::GPIO_ENABLE_W1TC, 1 << self.pin) };
        } else {
            unsafe { write_volatile(gpio_mux::GPIO_ENABLE1_W1TC, 1 << (self.pin - 32)) };
        }

        let io_pin = io_mux::io_mux_reg(self.pin);
        let mut val = unsafe { read_volatile(io_pin) };

        val = (val & !0b111) | io_mux::MCU_SEL_GPIO;

        val |= io_mux::FUN_IE;

        val &= !(io_mux::FUN_WPU | io_mux::FUN_WPD);

        unsafe {
            write_volatile(io_pin, val);
        }
    }

    pub fn into_output(self) -> Pin<Output> {
        let out = Pin {
            pin: self.pin,
            _mode: PhantomData,
        };
        out.config_output();
        out
    }
}

impl Pin<Output> {
    pub fn new(pin: u8) -> Self {
        assert!(pin <= 39, "Pin izvan domene 0-39");
        let p = Pin {
            pin,
            _mode: PhantomData,
        };
        p.config_output();
        p
    }

    fn config_output(&self) {
        if self.pin < 32 {
            unsafe { write_volatile(gpio_mux::GPIO_ENABLE_W1TS, 1 << self.pin) };
        } else {
            unsafe { write_volatile(gpio_mux::GPIO_ENABLE1_W1TS, 1 << (self.pin - 32)) };
        }

        let io_pin = io_mux::io_mux_reg(self.pin);
        let mut val = unsafe { read_volatile(io_pin) };

        val = (val & !0b111) | io_mux::MCU_SEL_GPIO;

        val &= !io_mux::FUN_IE;

        val &= !(io_mux::FUN_WPU | io_mux::FUN_WPD);

        unsafe { write_volatile(io_pin, val) };
    }

    pub fn set_high(&self) {
        if self.pin < 32 {
            unsafe { write_volatile(gpio_mux::GPIO_OUT_W1TS, 1 << self.pin) };
        } else {
            unsafe { write_volatile(gpio_mux::GPIO_OUT1_W1TS, 1 << (self.pin - 32)) };
        }
    }

    pub fn set_low(&self) {
        if self.pin < 32 {
            unsafe { write_volatile(gpio_mux::GPIO_OUT_W1TC, 1 << self.pin) };
        } else {
            unsafe { write_volatile(gpio_mux::GPIO_OUT1_W1TC, 1 << (self.pin - 32)) };
        }
    }

    pub fn into_input(self) -> Pin<Input> {
        let inp = Pin {
            pin: self.pin,
            _mode: PhantomData,
        };
        inp.config_input();
        inp
    }
}

impl<MODE> Pin<MODE> {
    pub fn set_pull(&self, pull: Pull) {
        let io_pin: *mut u32 = io_mux::io_mux_reg(self.pin);
        let mut val = unsafe { read_volatile(io_pin) };
        val &= !(io_mux::FUN_WPD | io_mux::FUN_WPU);
        match pull {
            Pull::Up => val |= io_mux::FUN_WPU,
            Pull::Down => val |= io_mux::FUN_WPD,
            Pull::None => {}
        }
        unsafe { write_volatile(io_pin, val) };
    }

    pub fn is_high(&self) -> bool {
        if self.pin < 32 {
            (unsafe { read_volatile(gpio_mux::GPIO_IN) } & (1 << self.pin)) != 0
        } else {
            (unsafe { read_volatile(gpio_mux::GPIO_IN1) } & (1 << self.pin - 32)) != 0
        }
    }

    pub fn get_pin(&self) -> u8 {
        self.pin
    }
}

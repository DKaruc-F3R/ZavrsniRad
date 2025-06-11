
use crate::{gpio_mux, io_mux, Pull};

use core::ptr::{read_volatile, write_volatile};

const IN_SEL_REG_BASE: usize = 0x3FF4_4130;
const OUT_SEL_REG_BASE: usize = 0x3FF4_4530;

pub fn peripheral_input_signal(pin: u8, per_sig: u8, pull: Pull) {
    unsafe {
        let in_sel_reg = (IN_SEL_REG_BASE + 4 * per_sig as usize) as *mut u32;

        write_volatile(in_sel_reg, (1 << 31) | (pin as u32 & 0x3F));

        let out_sel_reg = (OUT_SEL_REG_BASE + 4 * pin as usize) as *mut u32;
        let mut val = read_volatile(out_sel_reg);
        val |= 1 << 10; // mozda na 11
        write_volatile(out_sel_reg, val);

        if pin < 32 {
            write_volatile(gpio_mux::GPIO_ENABLE_W1TC, 1 << pin);
        } else {
            write_volatile(gpio_mux::GPIO_ENABLE1_W1TC, 1 << (pin - 32));
        }

        let io_mux_reg = io_mux::io_mux_reg(pin);
        let mut m = read_volatile(io_mux_reg);

        m = (m & !0b111) | io_mux::MCU_SEL_GPIO;

        m |= io_mux::FUN_IE;

        m &= !(io_mux::FUN_WPU | io_mux::FUN_WPD);

        match pull {
            Pull::Up => m |= io_mux::FUN_WPU,
            Pull::Down => m |= io_mux::FUN_WPD,
            Pull::None => {}
        }

        write_volatile(io_mux_reg, m);
    }
}

pub fn peripheral_output_signal(
    pin: u8,
    per_sig: u8,
    open_drain: bool,
    drive_strenght: u8,
) {
     unsafe {
        let out_sel_reg = (OUT_SEL_REG_BASE + (pin as usize) * 4) as *mut u32;
        let mut v = read_volatile(out_sel_reg);

        v = (v & !0x1FF) | (per_sig as u32 & 0x1FF);
        v |= 1 << 10; //mozda na 11
        write_volatile(out_sel_reg, v);

        if pin < 32 {
            write_volatile(gpio_mux::GPIO_ENABLE_W1TS, 1 << pin);
        } else {
            write_volatile(gpio_mux::GPIO_ENABLE1_W1TS, 1 << (pin - 32));
        }

        let pin_reg = (gpio_mux::GPIO_BASE + 0x88 + (pin as usize) * 4) as *mut u32;
        let mut drain = read_volatile(pin_reg);
        if open_drain {
            drain |= 1 << 2;
        } else {
            drain &= !(1 << 2);
        }
        write_volatile(pin_reg, drain);

        let io_mux_reg = io_mux::io_mux_reg(pin);
        let mut m = read_volatile(io_mux_reg);
        m = (m & !0b111) | io_mux::MCU_SEL_GPIO;

        m = (m & !(0b111 << 12)) | (((drive_strenght as u32) & 0b111) << 10); // mozda na 12 ovisi 

        if open_drain {
            m |= io_mux::FUN_WPU;
            m &= !io_mux::FUN_WPD;
        } else {
            m &= !(io_mux::FUN_WPU | io_mux::FUN_WPD);
        }
        write_volatile(io_mux_reg, m);
    }
}

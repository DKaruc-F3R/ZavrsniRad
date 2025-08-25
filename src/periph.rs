
use crate::{gpio_mux, io_mux, Pull};

use core::ptr::{read_volatile, write_volatile};

const IN_SEL_REG_BASE: usize = 0x3FF4_4130;
const OUT_SEL_REG_BASE: usize = 0x3FF4_4530;

pub fn peripheral_input_signal(pin: u8, per_sig: u8, pull: Pull) {
    unsafe {
        
        // Odredi IN_SEL registar za periferijski ulaz i zapiši pin kao izvor signala
        let in_sel_reg = (IN_SEL_REG_BASE + 4 * per_sig as usize) as *mut u32;
        write_volatile(in_sel_reg, (1 << 31) | (pin as u32 & 0x3F));

        // Ažuriraj OUT_SEL registar povezan s pinom
        let out_sel_reg = (OUT_SEL_REG_BASE + 4 * pin as usize) as *mut u32;
        let mut val = read_volatile(out_sel_reg);
        val |= 1 << 10;     // Ažuriraj OUT_SEL kontrolne bitove
        write_volatile(out_sel_reg, val);


        // Onemogući lokalni GPIO driver za taj pin
        if pin < 32 {
            write_volatile(gpio_mux::GPIO_ENABLE_W1TC, 1 << pin);
        } else {
            write_volatile(gpio_mux::GPIO_ENABLE1_W1TC, 1 << (pin - 32));
        }

        // Konfiguriraj IO_MUX, odaberi GPIO funkciju i omogući ulazni buffer
        let io_mux_reg = io_mux::io_mux_reg(pin);
        let mut m = read_volatile(io_mux_reg);

        // MCU_SEL = GPIO
        m = (m & !0b111) | io_mux::MCU_SEL_GPIO;

        // uključi ulaz (FUN_IE)
        m |= io_mux::FUN_IE;

        // Očisti postojeće pull bitove
        m &= !(io_mux::FUN_WPU | io_mux::FUN_WPD);

        match pull {
            Pull::Up => m |= io_mux::FUN_WPU,
            Pull::Down => m |= io_mux::FUN_WPD,
            Pull::None => {}
        }
        // Zapiši konfiguraciju natrag u IO_MUX registar
        write_volatile(io_mux_reg, m);
    }
}

pub fn peripheral_output_signal(pin: u8, per_sig: u8, open_drain: bool, drive_strenght: u8) {
     unsafe {

        // Ažuriraj OUT_SEL registar tako da periferni izlaz bude mapiran na fizički GPIO pin
        let out_sel_reg = (OUT_SEL_REG_BASE + (pin as usize) * 4) as *mut u32;
        let mut v = read_volatile(out_sel_reg);

        // Postavi novi identifikator signala (pogledati tablici u TRM za id. od signala)
        v = (v & !0x1FF) | (per_sig as u32 & 0x1FF);
        // postavi OUT_SEL
        v |= 1 << 10; //mozda na 11
        write_volatile(out_sel_reg, v);

        // Omogući lokalni GPIO driver
        if pin < 32 {
            write_volatile(gpio_mux::GPIO_ENABLE_W1TS, 1 << pin);
        } else {
            write_volatile(gpio_mux::GPIO_ENABLE1_W1TS, 1 << (pin - 32));
        }

        // Konfiguriraj open-drain
        let pin_reg = (gpio_mux::GPIO_BASE + 0x88 + (pin as usize) * 4) as *mut u32;
        let mut drain = read_volatile(pin_reg);
        if open_drain {
            drain |= 1 << 2;
        } else {
            drain &= !(1 << 2);
        }
        write_volatile(pin_reg, drain);

        // Konfiguriraj IO_MUX, postavi MCU_SEL na GPIO i postavljanje jačine izvoda
        let io_mux_reg = io_mux::io_mux_reg(pin);
        let mut m = read_volatile(io_mux_reg);
        // MCU_SEL = GPIO
        m = (m & !0b111) | io_mux::MCU_SEL_GPIO;

        // postavi drive-strength
        m = (m & !(0b111 << 12)) | (((drive_strenght as u32) & 0b111) << 10); 

        if open_drain {
            m |= io_mux::FUN_WPU;
            m &= !io_mux::FUN_WPD;
        } else {
            m &= !(io_mux::FUN_WPU | io_mux::FUN_WPD);
        }

        // Zapiši konfiguraciju natrag u IO_MUX registar
        write_volatile(io_mux_reg, m);
    }
}



pub const GPIO_BASE: usize = 0x3FF4_4000;

    //GPIO pins [0,31]
pub const GPIO_OUT_W1TS: *mut u32 =     (GPIO_BASE + 0x08) as *mut u32; // GPIO 0-31 izlazni registar_W1TS
pub const GPIO_OUT_W1TC: *mut u32 =     (GPIO_BASE + 0x0C) as *mut u32; // GPIO 0-31 izlazni registar_W1TC
pub const GPIO_ENABLE_W1TS: *mut u32 =  (GPIO_BASE + 0x24) as *mut u32; // GPIO 0-31 izlazni bit set registar
pub const GPIO_ENABLE_W1TC: *mut u32 =  (GPIO_BASE + 0x28) as *mut u32; // GPIO 0-31 izlazni bit clear registar
pub const GPIO_IN: *mut u32 =           (GPIO_BASE + 0x3C) as *mut u32; // GPIO 0-31 ulazni registar

    //GPIO pins [32,39]
pub const GPIO_OUT1_W1TS: *mut u32 =    (GPIO_BASE + 0x14) as *mut u32; // GPIO 32-39 izlazni registar_W1TS
pub const GPIO_OUT1_W1TC: *mut u32 =    (GPIO_BASE + 0x18) as *mut u32; // GPIO 32-39 izlazni registar_W1TC
pub const GPIO_ENABLE1_W1TS: *mut u32 = (GPIO_BASE + 0x30) as *mut u32; // GPIO 32-39 izlazni bit set registar
pub const GPIO_ENABLE1_W1TC: *mut u32 = (GPIO_BASE + 0x34) as *mut u32; // GPIO 32-39 izlazni bit clear registar
pub const GPIO_IN1: *mut u32 =          (GPIO_BASE + 0x40) as *mut u32; // GPIO 32-39 ulazni registar

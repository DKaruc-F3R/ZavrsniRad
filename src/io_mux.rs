

// Register 4.34. IO_MUX_x_REG bitovi

//pub const MCU_SEL: u32 = 0b111;
pub const MCU_SEL_GPIO: u32 = 0x2; //IO_MUX funckija za signal 2 je u svim pinovima GPIO


// Register 4.34. IO_MUX_x_REG bitovi
pub const FUN_IE: u32 = 1<<9; //input enable
pub const FUN_WPU: u32 = 1<<8; // pull up 
pub const FUN_WPD: u32 = 1<<7;// pul down



/// IO-MUX pad-config za ESP32 (pinovi 0–35), prema Table 4-6 (IO_MUX register summary).
///
/// Vraća adresu IO_MUX_<PAD>_REG za zadani GPIO pin.
/// Panica ako taj pin nema pad-config registar ili je izvan 0–39.
///

pub fn io_mux_reg(pin: u8) -> *mut u32 {
    let addr = match pin {
        0 => 0x3FF4_9044, // IO_MUX_GPIO0_REG
        1 => 0x3FF4_9088, // IO_MUX_U0TXD_REG (pin1 = U0TXD)
        2 => 0x3FF4_9040, // IO_MUX_GPIO2_REG
        3 => 0x3FF4_9084, // IO_MUX_U0RXD_REG (pin3 = U0RXD)
        4 => 0x3FF4_9048, // IO_MUX_GPIO4_REG
        5 => 0x3FF4_906C, // IO_MUX_GPIO5_REG

        //Pinovi 6-11 fizički SDIO/SPI-flash linije
        //Nemogu se rekonfigurirati za generični GPIO

        // SD-IO flash pads (ako se koriste kao GPIO)
        12 => 0x3FF4_9034, // IO_MUX_MTDI_REG (GPIO12)
        13 => 0x3FF4_9038, // IO_MUX_MTCK_REG (GPIO13)
        14 => 0x3FF4_9030, // IO_MUX_MTMS_REG (GPIO14)
        15 => 0x3FF4_903C, // IO_MUX_MTDO_REG (GPIO15)

        16 => 0x3FF4_904C, // IO_MUX_GPIO16_REG
        17 => 0x3FF4_9050, // IO_MUX_GPIO17_REG
        18 => 0x3FF4_9070, // IO_MUX_GPIO18_REG
        19 => 0x3FF4_9074, // IO_MUX_GPIO19_REG
        20 => 0x3FF4_9078, // IO_MUX_GPIO20_REG
        21 => 0x3FF4_907C, // IO_MUX_GPIO21_REG
        22 => 0x3FF4_9080, // IO_MUX_GPIO22_REG
        23 => 0x3FF4_908C, // IO_MUX_GPIO23_REG

        25 => 0x3FF4_9024, // IO_MUX_GPIO25_REG
        26 => 0x3FF4_9028, // IO_MUX_GPIO26_REG
        27 => 0x3FF4_902C, // IO_MUX_GPIO27_REG

        //Pinovi 28-31 služe isključivo za interne periferijske signale (I2S, I2CEXT, PWM sync)
        //Zasad njih nećemo koristi jer će se svi periferijski signali biti preko GPIO matrice

        32 => 0x3FF4_901C, // IO_MUX_GPIO32_REG
        33 => 0x3FF4_9020, // IO_MUX_GPIO33_REG
        34 => 0x3FF4_9014, // IO_MUX_GPIO34_REG (input-only)
        35 => 0x3FF4_9018, // IO_MUX_GPIO35_REG (input-only)
        36 => 0x3FF4_9004, // IO_MUX_GPIO36_REG (input-only)
        37 => 0x3FF4_9008, // IO_MUX_GPIO37_REG (input-only)
        38 => 0x3FF4_900C, // IO_MUX_GPIO38_REG (input-only)
        39 => 0x3FF4_9010, // IO_MUX_GPIO39_REG (input-only)

        _ => panic!("IO_MUX pad-config nije dostupan za pin {}", pin),
    };
    addr as *mut u32
}

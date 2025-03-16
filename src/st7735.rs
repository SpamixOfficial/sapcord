use display_interface_spi::SPIInterface;
use embedded_hal_1::{digital::OutputPin, spi::SpiDevice};
use core::convert::TryInto;
use embedded_graphics::{
    pixelcolor::{Rgb565, GrayColor},
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};

pub struct ST7735<SPI: SpiDevice, DC: OutputPin> {
    spi: SPIInterface<SPI, DC>,
    bl: DC,
    rst: DC,
    framebuffer: [u8; 180*128]
}

impl<SPI, DC> ST7735<SPI, DC>
where
    SPI: SpiDevice,
    DC: OutputPin,
{
    /// Initialize display "driver" with SPI, Data/Command pin, backlight pin and reset pin
    /// 
    /// NOTE: In case of sprig screen-change, please update struct!!
    pub fn init(spi:SPIInterface<SPI, DC>,  bl: DC, rst: DC) -> Self {
        Self {
            spi,
            bl,
            rst,
            framebuffer: [0; 180*128]
        }
    }
}

impl DrawTarget for ST7735<SPI: SpiDevice, DC: OutputPin> {
    type Color = Rgb565;
}

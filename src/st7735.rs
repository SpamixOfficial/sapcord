use display_interface_spi::SPIInterface;
use embedded_hal_1::{digital::OutputPin, spi::SpiDevice};
pub struct ST7735<SPI: SpiDevice, DC: OutputPin> {
    spi: SPIInterface<SPI, DC>,
    bl: DC,
    rst: DC
}

impl<SPI, DC> ST7735<SPI, DC>
where
    SPI: SpiDevice,
    DC: OutputPin,
{
    /// Initialize display "driver" with SPI, Data/Command pin, backlight pin and reset pin
    pub fn init(spi: SPI, dcx: DC, bl: DC, rst: DC) -> Self {
        let spi_interface = SPIInterface::new(spi, dcx);
        Self {
            spi: spi_interface,
            bl,
            rst
        }
    }
}

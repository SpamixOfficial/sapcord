#![no_std]
#![no_main]

mod controls;
mod st7735;
use core::cell::RefCell;

use controls::{Button, Controls};
use cyw43_pio::PioSpi;
use defmt::*;
use display_interface_spi::SPIInterface;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::spi::Phase;
use embassy_rp::spi::{Config as SpiConfig, Polarity, Spi};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Delay, Duration, Timer};
use embedded_graphics::mono_font::ascii::{FONT_10X20, FONT_5X8};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use mipidsi::options::{Orientation, Rotation};
use mipidsi::Builder;
use mipidsi::models::ST7735s;
use st7735::ST7735;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let fw = include_bytes!("../assets/firmware/43439A0.bin");
    let clm = include_bytes!("../assets/firmware/43439A0_clm.bin");

    // To make flashing faster for development, you may want to flash the firmwares independently
    // at hardcoded addresses, instead of baking them into the program with `include_bytes!`:
    //     probe-rs download ../../cyw43-firmware/43439A0.bin --binary-format bin --chip RP2040 --base-address 0x10100000
    //     probe-rs download ../../cyw43-firmware/43439A0_clm.bin --binary-format bin --chip RP2040 --base-address 0x10140000
    //let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 230321) };
    //let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let cywspi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (_net_device, mut control, runner) = cyw43::new(state, pwr, cywspi, fw).await;
    unwrap!(spawner.spawn(cyw43_task(runner)));
    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    // Non-complicated setup starts here (goddamn that's a lot of abstractions)

    let mut status_led = Output::new(p.PIN_28, Level::Low); // NOTE TO SELF: PIN_XX
                                                            // actually means GPIO_XX
                                                            // DO NOT CONFUSE WITH
                                                            // ACTUAL PINS!
                                                            // Set up controller
    let mut controls = Controls::init(
        p.PIN_5, p.PIN_6, p.PIN_7, p.PIN_8, p.PIN_12, p.PIN_13, p.PIN_14, p.PIN_15,
    );

    // Set up SPI interfaces
    // Display SPI
    let mut display_config = SpiConfig::default();
    display_config.frequency = 30000000;
    display_config.phase = Phase::CaptureOnSecondTransition;
    display_config.polarity = Polarity::IdleHigh;

    let spi = Spi::new_blocking(p.SPI0, p.PIN_18, p.PIN_19, p.PIN_16, display_config.clone());
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    let display_spi =
        SpiDeviceWithConfig::new(&spi_bus, Output::new(p.PIN_20, Level::High), display_config);

    let dcx = Output::new(p.PIN_22, Level::Low);
    let mut backlight = Output::new(p.PIN_17, Level::Low);
    let display_reset = Output::new(p.PIN_26, Level::Low);
    let display = ST7735::init(spi, dcx, bl, rst);

    status_led.set_high();
    info!("Everything went fine!");
    Timer::after_millis(50).await;
    status_led.set_low();

    loop {
        controls.check_for_input().await;
        match controls.pressed_button {
            Button::W => info!("W"),
            Button::A => info!("A"),
            Button::S => info!("S"),
            Button::D => info!("D"),
            Button::I => info!("I"),
            Button::J => info!("J"),
            Button::K => info!("K"),
            Button::L => info!("L"),
            _ => (),
        };
        //Timer::after(delay).await;
    }
}

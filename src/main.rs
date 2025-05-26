#![no_std]
#![no_main]

mod bsp;

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_time::Timer;
use panic_halt as _;

use crate::bsp::{AssignedResources, DmaResources, GpioResources, I2cResources, LedResources, LoraResources, SpiResources, UartResources, UsbResources};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
});

#[embassy_executor::task]
pub async fn logging(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let board = split_resources!(p);
    let usb_driver = Driver::new(board.usb.usb, Irqs);
    spawner.must_spawn(logging(usb_driver));

    loop {
        log::info!("iterate");
        Timer::after_secs(2).await;
    }
}
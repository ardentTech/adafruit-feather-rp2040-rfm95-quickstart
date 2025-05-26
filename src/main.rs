#![no_std]
#![no_main]

mod bsp;

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::rtc::{DateTime, DayOfWeek, Rtc};
use embassy_rp::usb::Driver;
use embassy_time::Timer;
use panic_halt as _;

//use crate::bsp::{AssignedResources, DmaResources, GpioResources, I2cResources, LedResources, LoraResources, RtcResources, SpiResources, UartResources, UsbResources};

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

    //let board = split_resources!(p);

    let usb_driver = Driver::new(p.USB, Irqs);
    spawner.must_spawn(logging(usb_driver));

    let mut rtc = Rtc::new(p.RTC);
    if !rtc.is_running() {
        log::info!("Start RTC");
        let now = DateTime {
            year: 2000,
            month: 1,
            day: 1,
            day_of_week: DayOfWeek::Saturday,
            hour: 0,
            minute: 0,
            second: 0,
        };
        rtc.set_datetime(now).unwrap();
    }

    Timer::after_millis(20000).await;

    if let Ok(dt) = rtc.now() {
        log::info!(
            "Now: {}-{:02}-{:02} {}:{:02}:{:02}",
            dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second,
        );
    }

    loop {
        log::info!("iterate");
        Timer::after_secs(2).await;
    }
}
#![no_std]
#![no_main]

mod bsp;

use core::cell::RefCell;
use cortex_m::asm::{wfe, wfi};
use cortex_m::interrupt::Mutex;
use embassy_executor::Spawner;
use embassy_rp::{bind_interrupts, interrupt};
use embassy_rp::gpio::{Level, Output};
use embassy_rp::pac::i2c::vals::Speed;
use embassy_rp::peripherals::USB;
use embassy_rp::rtc::{DateTime, DateTimeFilter, DayOfWeek, Rtc};
use embassy_rp::usb::Driver;
use embassy_time::Timer;
use panic_halt as _;

static LED: Mutex<RefCell<Option<Output<'static>>>> = Mutex::new(RefCell::new(None));
static RTC: Mutex<RefCell<Option<Rtc<embassy_rp::peripherals::RTC>>>> = Mutex::new(RefCell::new(None));

// TODO can this work with logging?

// bind_interrupts!(struct Irqs {
//     USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
// });

// #[embassy_executor::task]
// pub async fn logging(driver: Driver<'static, USB>) {
//     embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
// }

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    //let board = split_resources!(p);

    // let usb_driver = Driver::new(p.USB, Irqs);
    // spawner.must_spawn(logging(usb_driver));

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

    let led = Output::new(p.PIN_13, Level::Low);
    cortex_m::interrupt::free(|cs| {
        rtc.schedule_alarm(
            DateTimeFilter::default()
                .second(5)
        );
        LED.borrow(cs).borrow_mut().replace(led);
        RTC.borrow(cs).borrow_mut().replace(rtc);
        unsafe {
            cortex_m::peripheral::NVIC::unmask(interrupt::RTC_IRQ);
        }
    });

    loop {
        wfi(); // TODO vs wfe();
    }
}

#[interrupt]
fn RTC_IRQ() {
    cortex_m::interrupt::free(|cs| {
        let mut rtc = RTC.borrow(cs).borrow_mut();
        let rtc = rtc.as_mut().unwrap();
        let mut led = LED.borrow(cs).borrow_mut();
        let led = led.as_mut().unwrap();

        led.set_high();

        rtc.clear_interrupt();
    });
}
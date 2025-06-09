#![no_std]
#![no_main]

use core::cell::RefCell;
use cortex_m::asm::wfi;
use embassy_executor::Spawner;
use embassy_rp::interrupt;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::rtc::{DateTime, DateTimeFilter, DayOfWeek, Rtc};
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use panic_halt as _;

// "A mutex that allows borrowing data across executors and interrupts."
static RTC: Mutex<CriticalSectionRawMutex, RefCell<Option<Rtc<embassy_rp::peripherals::RTC>>>> = Mutex::new(RefCell::new(None));

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut real_time_clock = Rtc::new(p.RTC);
    if !real_time_clock.is_running() {
        let now = DateTime {
            year: 2000,
            month: 1,
            day: 1,
            day_of_week: DayOfWeek::Saturday,
            hour: 0,
            minute: 0,
            second: 0,
        };
        real_time_clock.set_datetime(now).unwrap();
    }
    schedule_alarm(&mut real_time_clock);
    RTC.lock(|r| r.borrow_mut().replace(real_time_clock));

    let mut led = Output::new(p.PIN_13, Level::Low);

    unsafe {
        cortex_m::peripheral::NVIC::unmask(interrupt::RTC_IRQ);
    }

    loop {
        wfi();
        led.toggle();
    }
}

#[interrupt]
fn RTC_IRQ() {
    critical_section::with(|cs| {
        let mut rtc = RTC.borrow(cs).borrow_mut();
        let rtc = rtc.as_mut().unwrap();
        rtc.clear_interrupt();
        schedule_alarm(rtc);
    });
}

fn schedule_alarm(rtc: &mut Rtc<embassy_rp::peripherals::RTC>) {
    rtc.schedule_alarm(
        // trigger alarm on 0 second of every minute
        DateTimeFilter::default().second(0)
    );
}
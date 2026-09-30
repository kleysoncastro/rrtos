//fase 0 base RTOS

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};


use critical_section::Mutex;
use defmt::info;
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
    handler,
    main,
    time::Duration,
    timer::timg::{Timer as TimgTimer, TimerGroup},
    timer::Timer as _,
};
use esp_println as _;

mod panic;

esp_bootloader_esp_idf::esp_app_desc!(); // boot laoder espressif default


static TICK_COUNT: AtomicU32 = AtomicU32::new(0);


const TICK_PERIOD: Duration = Duration::from_millis(100);


static SHARED: Mutex<RefCell<Option<Shared>>> = Mutex::new(RefCell::new(None));


struct Shared {
    timer: TimgTimer<'static>,
    led: Output<'static>,
}

#[handler]
fn timer_isr() {
    critical_section::with(|cs| {
        let mut shared = SHARED.borrow_ref_mut(cs);
        if let Some(Shared { timer, led }) = shared.as_mut() {
            timer.clear_interrupt();
            timer.load_value(TICK_PERIOD).ok();
            timer.start();

            led.toggle();
        }
    });

    TICK_COUNT.fetch_add(1, Ordering::Relaxed);
}


#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    info!("fase-0: boot ok, config gio");
    
    let led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    let timer_group0 = TimerGroup::new(peripherals.TIMG0);
    let timer0 = timer_group0.timer0;

    timer0.set_interrupt_handler(timer_isr);
    timer0.load_value(TICK_PERIOD).expect("invalid period");
    timer0.start();
    timer0.enable_interrupt(true);

    critical_section::with(|cs| {
        SHARED.borrow_ref_mut(cs).replace(Shared { timer: timer0, led })
    });

    info!("fase-0: timer run in loop main");


    loop {
        let current = TICK_COUNT.load(Ordering::Relaxed);
        info!("{}", current);
    }
}

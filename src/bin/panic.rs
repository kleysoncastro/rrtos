
use core::sync::atomic::Ordering;
use defmt::error;
use esp_hal::time::{Duration, Instant};

use crate::{SHARED, TICK_COUNT};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("PANIC: {}", defmt::Debug2Format(info));
    error!("ttick_count at the time of panic: {}", TICK_COUNT.load(Ordering::Relaxed));

    loop {
        critical_section::with(|cs| {
            if let Some(shared) = SHARED.borrow_ref_mut(cs).as_mut() {
                shared.led.toggle();
            }
        });

        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(50) {}
    }
}

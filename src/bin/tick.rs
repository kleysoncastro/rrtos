// Phase 2 — periodic timer that directs pre-empção.

use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    interrupt::{DirectBindableCpuInterrupt, Priority, enable_direct},
    peripherals::{GPIO, GPIO4, Interrupt, SYSTIMER},
    time::Duration,
    timer::{PeriodicTimer, systimer::SystemTimer},
};

use crate::context_switch::_rrtos_tick_isr;

pub const TICK_MS: u64 = 1;
const PROBE_PIN: u32 = 4;

static mut TIMER: Option<PeriodicTimer<'static, esp_hal::Blocking>> = None;

pub fn init(systimer: SYSTIMER<'static>, probe: GPIO4<'static>) {

    let _ = Output::new(probe, Level::Low, OutputConfig::default());

    let alarm = SystemTimer::new(systimer).alarm0;
    let timer = PeriodicTimer::new(alarm);
    unsafe { TIMER = Some(timer) };

    enable_direct(
        Interrupt::SYSTIMER_TARGET0,
        Priority::Priority1,
        DirectBindableCpuInterrupt::Interrupt0,
        _rrtos_tick_isr,
    );
}

/// Arms the periodic alarm and enables non-peripheral interruption.
#[allow(clippy::deref_addrof)]
pub fn start() {
    unsafe {
        let timer = (*(&raw mut TIMER)).as_mut().unwrap();
        timer.start(Duration::from_millis(TICK_MS)).unwrap();
        timer.listen();
    }
}

/// Clear the alarm0 interruption request.
#[inline(always)]
pub fn ack() {
    SYSTIMER::regs().int_clr().write(|w| w.target0().clear_bit_by_one());
}

#[inline(always)]
pub fn probe_high() {
    GPIO::regs().out_w1ts().write(|w| unsafe { w.bits(1 << PROBE_PIN) });
}

#[inline(always)]
pub fn probe_low() {
    GPIO::regs().out_w1tc().write(|w| unsafe { w.bits(1 << PROBE_PIN) });
}

//stap-2 0 base RTOS
//
// A periodic timer of 1 ms (SYSTIMER) chama or scheduler within the ISR,
// and a change of context happens there. Nenhuma tarefa chama `yield_now`;
// all of them are loops that never cooperate.
#![no_std]
#![no_main]



use core::{cell::RefCell, sync::atomic::AtomicU32};

use critical_section::Mutex;
use defmt::info;
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
    main,
    time::{Instant, Duration},
    timer::timg::{Timer as TimgTimer},
    
};
use esp_println as _;

mod panic;
mod context_switch;
mod scheduler;
mod task;
mod tick;

esp_bootloader_esp_idf::esp_app_desc!(); // boot laoder espressif default

struct Shared {
    timer: TimgTimer<'static>,
    led: Output<'static>,
}

static SHARED: Mutex<RefCell<Option<Shared>>> = Mutex::new(RefCell::new(None));
static TICK_COUNT: AtomicU32 = AtomicU32::new(0);

static LED_TASK0: Mutex<RefCell<Option<Output<'static>>>> = Mutex::new(RefCell::new(None));
static LED_TASK1: Mutex<RefCell<Option<Output<'static>>>> = Mutex::new(RefCell::new(None));
static LED_TASK2: Mutex<RefCell<Option<Output<'static>>>> = Mutex::new(RefCell::new(None));

static mut STACK0: task::TaskStack = task::TaskStack::new();
static mut STACK1: task::TaskStack = task::TaskStack::new();
static mut STACK2: task::TaskStack = task::TaskStack::new();


fn busy_delay_ms(ms: u64) {
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(ms) {}
}

fn toggle(led: &Mutex<RefCell<Option<Output<'static>>>>) {
    critical_section::with(|cs| {
        if let Some(led) = led.borrow_ref_mut(cs).as_mut() {
            led.toggle();
        }
    });
}

extern "C" fn task0_entry() -> ! {

    loop {
        info!("task 0 running");
        toggle(&LED_TASK0);
        busy_delay_ms(100);
    }
}

extern  "C" fn task1_entry() -> ! {
    loop {
        info!("Task 1 running");
        toggle(&LED_TASK1);
        busy_delay_ms(300);
        toggle(&LED_TASK1);
        busy_delay_ms(300);
        
    }
}

extern "C" fn task2_entry() -> ! {
    loop {
        info!("Task 2 running");
        toggle(&LED_TASK2);
        busy_delay_ms(100);
        toggle(&LED_TASK2);
        busy_delay_ms(100);
    }
}

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    info!("step-1: boot ok, initializing cooperative tasks");
    
    let led0 = Output::new(peripherals.GPIO10, Level::Low, OutputConfig::default());
    let led1 = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    let led2 = Output::new(peripherals.GPIO3, Level::Low, OutputConfig::default());

        critical_section::with(|cs| {
        LED_TASK0.borrow_ref_mut(cs).replace(led0);
        LED_TASK1.borrow_ref_mut(cs).replace(led1);
        LED_TASK2.borrow_ref_mut(cs).replace(led2);
    });

  
    unsafe {
        let sp0 = task::init_stack(&raw mut STACK0, task0_entry);
        let sp1 = task::init_stack(&raw mut STACK1, task1_entry);
        let sp2 = task::init_stack(&raw mut STACK2, task2_entry);

        scheduler::register_task(0, sp0, &raw mut STACK0 as usize);
        scheduler::register_task(1, sp1, &raw mut STACK1 as usize);
        scheduler::register_task(2, sp2, &raw mut STACK2 as usize);


        info!("step-1: tasks registered, handing over control to the scheduler");
        tick::init(peripherals.SYSTIMER, peripherals.GPIO4);
        scheduler::start();
        
    }
}

// step-2 — scheduler round-robin preemptiv


use crate::context_switch::_rrtos_restore_context;
use crate::task::{self, TaskControlBlock, TaskState};
use crate::tick;

pub const NUM_TASKS: usize = 3;

static mut TASKS: [TaskControlBlock; NUM_TASKS] = [TaskControlBlock::empty(); NUM_TASKS];
static mut STACK_BASES: [usize; NUM_TASKS] = [0; NUM_TASKS];
static mut CURRENT: usize = 0;

// Registers the TCB at `index` with an initial `sp` calculated by
// `task::init_stack`, and stores the base address of the stack for
// canary checks during the tick.
//
// # Safety
// Must be called once per task—for all `NUM_TASKS` tasks—before
// `start()` is invoked.
pub unsafe fn register_task(index: usize, initial_sp: usize, stack_base: usize) {
    unsafe {
        TASKS[index] = TaskControlBlock { sp: initial_sp, state: TaskState::Ready };
        STACK_BASES[index] = stack_base;
    }
}

// Hands over control definitively to task 0 and sets the tick.
pub unsafe fn start() -> ! {
    unsafe {
        core::arch::asm!("csrci mstatus, 8"); // MIE = 0
        TASKS[0].state = TaskState::Running;
        CURRENT = 0;
        tick::start();
        _rrtos_restore_context(TASKS[0].sp)
    }
}

// Tick body, called within an interrupt context.
pub unsafe extern "C" fn tick_rust(saved_sp: usize) -> usize {
    tick::ack();
    tick::probe_high();

    let next_sp = unsafe {
        let current = CURRENT;
        TASKS[current].sp = saved_sp;

        if !task::check_canary(STACK_BASES[current]) {
            panic!("stack overflow detectado na tarefa {}", current);
        }

        let next = (current + 1) % NUM_TASKS;
        TASKS[current].state = TaskState::Ready;
        TASKS[next].state = TaskState::Running;
        CURRENT = next;
        TASKS[next].sp
    };

    tick::probe_low();
    next_sp
}

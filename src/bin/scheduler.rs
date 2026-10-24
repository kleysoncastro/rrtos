

use crate::context_switch::switch_context;
use crate::task::{self, TaskControlBlock, TaskState};

pub const NUM_TASKS: usize = 3;

static mut TASKS: [TaskControlBlock; NUM_TASKS] = [TaskControlBlock::empty(); NUM_TASKS];
static mut STACK_BASES: [usize; NUM_TASKS] = [0; NUM_TASKS];
static mut CURRENT: usize = 0;


static mut BOOT_SP: usize = 0;

pub unsafe fn register_task(index: usize, initial_sp: usize, stack_base: usize) {
    unsafe {
        TASKS[index] = TaskControlBlock { sp: initial_sp, state: TaskState::Ready };
        STACK_BASES[index] = stack_base;
    }
}

pub unsafe fn start() -> ! {
    unsafe {
        TASKS[0].state = TaskState::Running;
        CURRENT = 0;
        switch_context(&raw mut BOOT_SP, TASKS[0].sp);
    }
    unreachable!("The boot flow is never resumed after the first switch_context")
}

pub fn yield_now() {
    unsafe {
        let current = CURRENT;
        let next = (current + 1) % NUM_TASKS;

        if !task::check_canary(STACK_BASES[current]) {
            panic!("stack overflow detected in task {}", current);
        }

        TASKS[current].state = TaskState::Ready;
        TASKS[next].state = TaskState::Running;
        CURRENT = next;

        let save_ptr = &raw mut TASKS[current].sp;
        let restore_sp = TASKS[next].sp;
        switch_context(save_ptr, restore_sp);
    }
}

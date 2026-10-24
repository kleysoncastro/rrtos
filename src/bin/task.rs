// step-2 — Minimal TCB (Task Control Block) and setup of the initial frame for each task.

pub const STACK_SIZE: usize = 2048;

use crate::context_switch::{
    FRAME_SIZE, GP_OFFSET_WORDS, MEPC_OFFSET_WORDS, MSTATUS_OFFSET_WORDS,
};

// Initial `mstatus` for a new task: MPP=11 (return to M-mode) and
// MPIE=1, so that the `mret` from the first dispatch enables interrupts
// (MIE <- MPIE). Without MPIE, the new task would run forever without a tick.
const INITIAL_MSTATUS: u32 = (0b11 << 11) | (1 << 7);

pub const STACK_CANARY: u32 = 0xC0FF_EEEE;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TaskState {
    Ready,
    Running,
}

#[derive(Clone, Copy)]
pub struct TaskControlBlock {
    pub sp: usize,
    pub state: TaskState,
}

impl TaskControlBlock {
    pub const fn empty() -> Self {
        Self { sp: 0, state: TaskState::Ready }
    }
}

#[repr(align(16))]
#[allow(dead_code)]
pub struct TaskStack(pub [u8; STACK_SIZE]);

impl TaskStack {
    pub const fn new() -> Self {
        Self([0; STACK_SIZE])
    }
}

pub unsafe fn init_stack(stack: *mut TaskStack, entry: extern "C" fn() -> !) -> usize {
    let stack_base = stack as usize;
    let stack_top = (stack_base + STACK_SIZE) & !0xF;
    let frame_addr = stack_top - FRAME_SIZE;

    unsafe {
        let frame = frame_addr as *mut u32;
        for i in 0..(FRAME_SIZE / 4) {
            frame.add(i).write(0);
        }
        
        let gp: u32;
        core::arch::asm!("mv {}, gp", out(reg) gp);
        frame.add(GP_OFFSET_WORDS).write(gp);
        frame.add(MEPC_OFFSET_WORDS).write(entry as usize as u32);
        frame.add(MSTATUS_OFFSET_WORDS).write(INITIAL_MSTATUS);

        (stack_base as *mut u32).write(STACK_CANARY);
    }

    frame_addr
}

pub fn check_canary(stack_base: usize) -> bool {
    unsafe { (stack_base as *const u32).read() == STACK_CANARY }
}

use core::arch::naked_asm;

/// Save the street-saved recorders from the current task on your own
/// stack, gravel or `sp` resulting in `*save_sp`, and restores the
/// callee-saved loggers starting from `restore_sp`.
///
/// #Safety
/// `save_sp` must be specified for a valid and grave `usize` (or field
/// `sp` do TCB Task Control Block da terefa current). `restore_sp` must be a value of `sp`
/// produced by `task::init_stack` or by a call before
/// `switch_context` for the same stack — any other value is
/// Guaranteed stack corruption.
#[unsafe(naked)]
pub unsafe extern "C" fn switch_context(save_sp: *mut usize, restore_sp: usize) {
    naked_asm!(
        "addi sp, sp, -64",
        "sw s11, 0(sp)",
        "sw s10, 4(sp)",
        "sw s9,  8(sp)",
        "sw s8,  12(sp)",
        "sw s7,  16(sp)",
        "sw s6,  20(sp)",
        "sw s5,  24(sp)",
        "sw s4,  28(sp)",
        "sw s3,  32(sp)",
        "sw s2,  36(sp)",
        "sw s1,  40(sp)",
        "sw s0,  44(sp)",
        "sw ra,  48(sp)",
        // Save the sp of the task that is being done (as the frame is 
        // discounted) no TCB dela, BEFORE overscrever sp.
        "sw sp, 0(a0)",
        // Stack change: from here we are physically in the stack 
        // gives next task.
        "mv sp, a1",
        "lw s11, 0(sp)",
        "lw s10, 4(sp)",
        "lw s9,  8(sp)",
        "lw s8,  12(sp)",
        "lw s7,  16(sp)",
        "lw s6,  20(sp)",
        "lw s5,  24(sp)",
        "lw s4,  28(sp)",
        "lw s3,  32(sp)",
        "lw s2,  36(sp)",
        "lw s1,  40(sp)",
        "lw s0,  44(sp)",
        "lw ra,  48(sp)",
        "addi sp, sp, 64",
        // For a task resumed from a previous yield, this is again 
        // for inside `yield_now`, logo after calling 
        // `switch_context`. For a new task, `ra` was forged by 
        // `task::init_stack` and this jumps directly to the entry point.
        "ret",
    );
}

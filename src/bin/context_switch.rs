//! Fase 2 — troca de contexto dentro da ISR.
//!
//! Diferente da Fase 1, não existe mais `switch_context` chamado pela
//! tarefa. A tarefa é interrompida a qualquer instrução, então a ISR
//! precisa salvar o contexto COMPLETO (não só os callee-saved): como a
//! tarefa não "chamou" nada, nenhum registrador é descartável.
//!
//! Layout do frame na stack da tarefa interrompida (144 bytes, mantém
//! `sp` alinhado em 16):
//!
//! ```text
//!   0..=120  x1, (x2 sem uso), x3..x31   -> offset = (n - 1) * 4
//!   124      mepc     (PC onde a tarefa foi interrompida)
//!   128      mstatus  (MPIE/MPP, para o mret reabilitar interrupções)
//!   132..143 padding
//! ```
//!
//! O slot de x2 (`sp`) fica sem uso: o `sp` salvo é o próprio endereço do
//! frame, guardado no TCB. `task::init_stack` monta um frame idêntico à
//! mão para tarefas novas, então a primeira entrada numa tarefa usa o
//! mesmo caminho de restauração de qualquer outra.

use core::arch::global_asm;

pub const FRAME_SIZE: usize = 144;
pub const MEPC_OFFSET_WORDS: usize = 31;
pub const MSTATUS_OFFSET_WORDS: usize = 32;
pub const GP_OFFSET_WORDS: usize = 2;

unsafe extern "C" {
    /// Entrada crua do trap, instalada no `mtvec` por `tick::init`.
    pub fn _rrtos_tick_isr();
    /// Restaura o frame apontado por `sp` e executa `mret`. Nunca retorna.
    pub fn _rrtos_restore_context(sp: usize) -> !;
}

global_asm!(
    ".section .trap, \"ax\"",
    ".balign 4",
    ".global _rrtos_tick_isr",
    "_rrtos_tick_isr:",
    "addi sp, sp, -{FRAME_SIZE}",
    "sw x1, 0(sp)",
    "sw x3, 8(sp)",
    "sw x4, 12(sp)",
    "sw x5, 16(sp)",
    "sw x6, 20(sp)",
    "sw x7, 24(sp)",
    "sw x8, 28(sp)",
    "sw x9, 32(sp)",
    "sw x10, 36(sp)",
    "sw x11, 40(sp)",
    "sw x12, 44(sp)",
    "sw x13, 48(sp)",
    "sw x14, 52(sp)",
    "sw x15, 56(sp)",
    "sw x16, 60(sp)",
    "sw x17, 64(sp)",
    "sw x18, 68(sp)",
    "sw x19, 72(sp)",
    "sw x20, 76(sp)",
    "sw x21, 80(sp)",
    "sw x22, 84(sp)",
    "sw x23, 88(sp)",
    "sw x24, 92(sp)",
    "sw x25, 96(sp)",
    "sw x26, 100(sp)",
    "sw x27, 104(sp)",
    "sw x28, 108(sp)",
    "sw x29, 112(sp)",
    "sw x30, 116(sp)",
    "sw x31, 120(sp)",
    "csrr t0, mepc",
    "sw t0, 124(sp)",
    "csrr t0, mstatus",
    "sw t0, 128(sp)",
    // a0 = sp do frame recém-salvo; tick_rust devolve em a0 o sp do
    // frame da próxima tarefa (pode ser o mesmo).
    "mv a0, sp",
    "call {tick}",
    // Cai direto em _rrtos_restore_context.
    ".global _rrtos_restore_context",
    "_rrtos_restore_context:",
    "mv sp, a0",
    "lw t0, 124(sp)",
    "csrw mepc, t0",
    "lw t0, 128(sp)",
    "csrw mstatus, t0",
    "lw x1, 0(sp)",
    "lw x3, 8(sp)",
    "lw x4, 12(sp)",
    "lw x6, 20(sp)",
    "lw x7, 24(sp)",
    "lw x8, 28(sp)",
    "lw x9, 32(sp)",
    "lw x10, 36(sp)",
    "lw x11, 40(sp)",
    "lw x12, 44(sp)",
    "lw x13, 48(sp)",
    "lw x14, 52(sp)",
    "lw x15, 56(sp)",
    "lw x16, 60(sp)",
    "lw x17, 64(sp)",
    "lw x18, 68(sp)",
    "lw x19, 72(sp)",
    "lw x20, 76(sp)",
    "lw x21, 80(sp)",
    "lw x22, 84(sp)",
    "lw x23, 88(sp)",
    "lw x24, 92(sp)",
    "lw x25, 96(sp)",
    "lw x26, 100(sp)",
    "lw x27, 104(sp)",
    "lw x28, 108(sp)",
    "lw x29, 112(sp)",
    "lw x30, 116(sp)",
    "lw x31, 120(sp)",
    "lw x5, 16(sp)",
    "addi sp, sp, {FRAME_SIZE}",
    "mret",
    FRAME_SIZE = const FRAME_SIZE,
    tick = sym crate::scheduler::tick_rust,
);

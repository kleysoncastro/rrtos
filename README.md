# rrtos

A small RTOS written in Rust for the **ESP32-C6** (RISC-V, single core).

> **This is a research and learning project.** It is not meant to compete with
> existing RTOSes, and it is not production software.

## Motivation

The goal here is **not to reinvent the wheel**. Mature RTOSes already exist
(FreeRTOS, Zephyr, Embassy, RTIC, ...). This project exists to understand, by
building it, how a **preemptive system** actually works under the hood:

- boot and bare-metal initialization
- context switching (cooperative first, then preemptive)
- timer-driven scheduling (round-robin)
- interrupts and critical sections
- synchronization primitives built on atomics

Every phase is small, self-contained and documented, so the focus stays on
fixing the concept rather than piling up code.

## Why the ESP32-C6?

- It was the board already available on my desk.
- It is **RISC-V** (RV32IMAC), an open ISA with plenty of documentation,
  specs and community material.
- The single-core variant keeps the first stages simple: no SMP concerns while
  learning the fundamentals.
- Good Rust support through `esp-hal` and the `riscv32imac-unknown-none-elf`
  target.

## Roadmap

| Phase | Topic                                             | Status |
|------|----------------------------------------------------|--------|
| 0    | Bare-metal boot and first interrupt                | Done   |
| 1    | Cooperative context switching between tasks        | Done   |
| 2    | Preemptive round-robin scheduling (timer tick)     | Done   |
| ...  | Synchronization primitives, priorities, etc.       | Planned |

> Each step is a branch; for example, you can easily access one using `git checkout step-1`.

### Future goals

- **Parallelism:** move from single core to multi-core / parallel execution
  and study the problems that come with it (memory ordering, locking,
  scheduling across cores).
- **Microkernel:** continue the studies towards a microkernel design, with
  isolation between components (the RISC-V PMP is a candidate building block).

## Tech stack

- Rust (edition 2024, `no_std`)
- [`esp-hal`](https://github.com/esp-rs/esp-hal)
- `defmt` for logging
- Target: `riscv32imac-unknown-none-elf`

## Building and running

Requirements:

- Rust toolchain (the target and `rust-src` are installed through
  `rust-toolchain.toml`)
- [`espflash`](https://github.com/esp-rs/espflash)
- An ESP32-C6 board connected over USB

```sh
cargo run --release
```

The configured runner flashes the board and opens the serial monitor with
`defmt` output.

## Status

Work in progress. Expect rough edges, rewrites and changing APIs, that is part
of the learning process.

## References

- *Rust Atomics and Locks*, Mara Bos
- RISC-V privileged and unprivileged specifications
- ESP32-C6 Technical Reference Manual

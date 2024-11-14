#![allow(unused)]

pub const PLIC_BASE: u64 = 0xc00_0000;
// The size of the PLIC memory-mapped region, calculated as:
// 0x200000: the base region size (2MB) for the PLIC registers
// (2 * 0x1000): additional space for interrupt contexts, where each context size is 4KB
// This accounts for interrupt contexts for a single core in M and S modes, each having a 4KB area
pub const PLIC_SIZE: u64 = (0x200000 + (1 * 2) * 0x1000);

pub const UART_BASE: u64 = 0x1000_0000;
pub const UART_SIZE: u64 = 0x100;

pub const DRAM_BASE: u64 = 0x8000_0000;
pub const DRAM_SIZE: u64 = 1024 * 1024 * 128;

// Machine Information Registers
/// Vendor ID.
pub const MVENDORID: u64 = 0xF11;
/// Architecture ID.
pub const MARCHID: u64 = 0xF12;
/// Implementation ID.
pub const MIMPID: u64 = 0xF13;
/// Hardware thread ID.
pub const MHARTID: u64 = 0xF14;
/// Pointer to configuration data structure.
pub const MCONFIGPTR: u64 = 0xF15;

// Machine Trap Setup
/// Machine status register.
pub const MSTATUS: u64 = 0x300;
/// ISA and extensions.
pub const MISA: u64 = 0x301;
/// Machine exception delegation register.
pub const MEDELEG: u64 = 0x302;
/// Machine interrupt delegation register.
pub const MIDELEG: u64 = 0x303;
/// Machine interrupt-enable register.
pub const MIE: u64 = 0x304;
/// Machine trap-handler base address.
pub const MTVEC: u64 = 0x305;
/// Machine counter enable.
pub const MCOUNTEREN: u64 = 0x306;
/// Additional machine status register (RV32 only).
pub const MSTATUSH: u64 = 0x310;
/// Upper 32 bits of MEDeleg (RV32 only).
pub const MEDELEGH: u64 = 0x312;

// Machine Trap Handling
/// Machine scratch register.
pub const MSCRATCH: u64 = 0x340;
/// Machine exception program counter.
pub const MEPC: u64 = 0x341;
/// Machine trap cause.
pub const MCAUSE: u64 = 0x342;
/// Machine trap value.
pub const MTVAL: u64 = 0x343;
/// Machine interrupt pending.
pub const MIP: u64 = 0x344;
/// Machine trap instruction (transformed).
pub const MTINST: u64 = 0x34A;
/// Machine second trap value.
pub const MTVAL2: u64 = 0x34B;

// Supervisor Trap Setup
/// Supervisor status register.
pub const SSTATUS: u64 = 0x100;
/// Supervisor interrupt-enable register.
pub const SIE: u64 = 0x104;
/// Supervisor trap handler base address.
pub const STVEC: u64 = 0x105;
/// Supervisor counter enable.
pub const SCOUINTEREN: u64 = 0x106;

// Supervisor Configuration
/// Supervisor environment configuration register.
pub const SENVCFG: u64 = 0x10A;

// Supervisor Counter Setup
/// Supervisor counter-inhibit register.
pub const SCOUNTINHIBIT: u64 = 0x120;

// Supervisor Trap Handling
/// Supervisor scratch register.
pub const SSCRATCH: u64 = 0x140;
/// Supervisor exception program counter.
pub const SEPC: u64 = 0x141;
/// Supervisor trap cause.
pub const SCAUSE: u64 = 0x142;
/// Supervisor trap value.
pub const STVAL: u64 = 0x143;
/// Supervisor interrupt pending.
pub const SIP: u64 = 0x144;
/// Supervisor count overflow.
pub const SCOUNTOVF: u64 = 0xDA0;

// Supervisor Protection and Translation
/// Supervisor address translation and protection.
pub const SATP: u64 = 0x180;

// Debug/Trace Registers
/// Supervisor-mode context register.
pub const SCONTEXT: u64 = 0x5A8;

// Unprivileged Counter/Timers
/// Cycle counter for RDCYCLE instruction.
pub const CYCLE: u64 = 0xC00;
/// Timer for RDTIME instruction.
pub const TIME: u64 = 0xC01;
/// Instructions-retired counter for RDINSTRET instruction.
pub const INSTRET: u64 = 0xC02;

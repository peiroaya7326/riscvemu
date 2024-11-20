use crate::bus::*;
use crate::csr::*;
use crate::exception::*;
use crate::interrupt::*;
use crate::lib::address::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    User,
    Supervisor,
    Machine,
}

impl Mode {
    pub const fn code(self) -> u64 {
        match self {
            Mode::User => 0b00,
            Mode::Supervisor => 0b01,
            Mode::Machine => 0b11,
        }
    }
}

pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,
    pub mode: Mode,
    pub csr: Csr,
    pub bus: Bus,
}

impl Cpu {
    pub fn new(binary: Vec<u8>) -> Self {
        let mut regs = [0; 32];
        regs[2] = (DRAM_BASE + DRAM_SIZE) as u64;
        Self {
            regs,
            pc: DRAM_BASE,
            mode: Mode::Machine,
            bus: Bus::new(binary),
            csr: Csr::new(),
        }
    }
    /// Load a value from a dram.
    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        self.bus.load(addr, size)
    }

    /// Store a value to a dram.
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        self.bus.store(addr, size, value)
    }

    pub fn fetch(&mut self) -> Result<u64, Exception> {
        match self.bus.load(self.pc, 32) {
            Ok(inst) => Ok(inst),
            Err(e) => Err(e),
        }
    }

    pub fn execute(&mut self, inst: u64) -> Result<(), Exception> {
        let op1 = inst & 0b11;
        let op2 = (inst >> 2) & 0b111;
        let op3 = (inst >> 5) & 0b11;
        let rd = (inst >> 7) & 0x1f;
        let rs1 = (inst >> 15) & 0x1f;
        let rs2 = (inst >> 20) & 0x1f;
        let funct3 = (inst >> 12) & 0x7;
        let funct7 = (inst >> 25) & 0x7f;

        let mut inst_step: u64 = match op1 {
            0b10 => 2,
            0b11 => 4,
            _ => return Err(Exception::IllegalInstruction(inst)),
        };

        self.regs[0] = 0;

        match (op3, op2) {
            // Group 0 (inst[6:5] == 00)
            (0b00, 0b000) => {
                // Load
                let imm = ((inst as i32 as i64) >> 20) as u64;
                let addr = self.regs[rs1 as usize].wrapping_add(imm);
                match funct3 {
                    0b000 => self.execute_lb(addr, rd)?,
                    0b001 => self.execute_lh(addr, rd)?,
                    0b010 => self.execute_lw(addr, rd)?,
                    0b011 => self.execute_ld(addr, rd)?,
                    0b100 => self.execute_lbu(addr, rd)?,
                    0b101 => self.execute_lhu(addr, rd)?,
                    0b110 => self.execute_lwu(addr, rd)?,
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }
            (0b00, 0b001) => {
                // LOAD-FP
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b00, 0b010) => {
                // custom-0
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b00, 0b011) => {
                // MISC-MEM
                // FENCE
                match funct3 {
                    0b000 => self.execute_fence(), // FENCE
                    // 0b001 => self.execute_fence_i(),  // FENCE.I
                    _ => return Err(Exception::IllegalInstruction(inst)),
                }
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b00, 0b100) => {
                // OP-IMM
                // imm[11:0] = inst[31:20]
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                let shamt = (imm & 0x3f) as u32;
                match (funct7, funct3) {
                    (_, 0b000) => self.execute_addi(rd, rs1, imm),
                    (_, 0b010) => self.execute_slti(rd, rs1, imm),
                    (_, 0b011) => self.execute_sltiu(rd, rs1, imm),
                    (_, 0b100) => self.execute_xori(rd, rs1, imm),
                    (_, 0b110) => self.execute_ori(rd, rs1, imm),
                    (_, 0b111) => self.execute_andi(rd, rs1, imm),
                    (0b000_0000, 0b001) => self.execute_slli(rd, rs1, shamt),
                    (0b000_0000, 0b101) => self.execute_srli(rd, rs1, shamt),
                    (0b010_0000, 0b101) => self.execute_srai(rd, rs1, shamt),
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }
            (0b00, 0b101) => {
                // AUIPC
                let imm = (inst & 0xfffff000) as i32 as i64 as u64;
                self.execute_auipc(rd, imm)
            }
            (0b00, 0b110) => {
                // OP-IMM32
                let imm = ((inst as i32 as i64) >> 20) as u64;

                // "SLLIW, SRLIW, and SRAIW encodings with imm[5] ̸= 0 are reserved."
                let shamt = (imm & 0x1f) as u32;
                match (funct7, funct3) {
                    (_, 0b000) => self.execute_addiw(rd, rs1, imm),
                    (0b000_0000, 0b001) => self.execute_slliw(rd, rs1, shamt),
                    (0b000_0000, 0b101) => self.execute_srliw(rd, rs1, shamt),
                    (0b010_0000, 0b101) => self.execute_sraiw(rd, rs1, shamt),
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }

            // Group 1 (inst[6:5] == 01)
            (0b01, 0b000) => {
                // Store
                // imm[11:5|4:0] = inst[31:25|11:7]
                let imm = (((inst & 0xfe000000) as i32 as i64 >> 20) as u64) | ((inst >> 7) & 0x1f);
                let addr = self.regs[rs1 as usize].wrapping_add(imm);
                match funct3 {
                    0b000 => self.execute_sb(addr, rs2)?,
                    0b001 => self.execute_sh(addr, rs2)?,
                    0b010 => self.execute_sw(addr, rs2)?,
                    0b011 => self.execute_sd(addr, rs2)?,
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }
            (0b01, 0b001) => {
                // STORE-FP
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b01, 0b010) => {
                // custom-1
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b01, 0b011) => {
                // AMO
                let _rl = (inst >> 25) & 1; // release access
                let _aq = (inst >> 26) & 1; // acquire access
                let t = (inst >> 27) & 0x1f;
                match (funct3, t) {
                    // (0b010, 0b00010) => self.execute_lr_w(inst),
                    // (0b010, 0b00011) => self.execute_sc_w(inst),
                    (0b010, 0b00001) => self.execute_amoswap_w(rd, rs1, rs2)?,
                    (0b010, 0b00000) => self.execute_amoadd_w(rd, rs1, rs2)?,
                    // (0b010, 0b00100) => self.execute_amoxor_w(inst),
                    // (0b010, 0b01100) => self.execute_amoand_w(inst),
                    // (0b010, 0b01000) => self.execute_amoor_w(inst),
                    // (0b010, 0b10000) => self.execute_amomin_w(inst),
                    // (0b010, 0b10100) => self.execute_amomax_w(inst),
                    // (0b010, 0b11000) => self.execute_amominu_w(inst),
                    // (0b010, 0b11100) => self.execute_amomaxu_w(inst),
                    // (0b011, 0b00010) => self.execute_lr_d(inst),
                    // (0b011, 0b00011) => self.execute_sc_d(inst),
                    (0b011, 0b00001) => self.execute_amoswap_d(rd, rs1, rs2)?,
                    (0b011, 0b00000) => self.execute_amoadd_d(rd, rs1, rs2)?,
                    // (0b011, 0b00100) => self.execute_amoxor_d(inst),
                    // (0b011, 0b01100) => self.execute_amoand_d(inst),
                    // (0b011, 0b01000) => self.execute_amoor_d(inst),
                    // (0b011, 0b10000) => self.execute_amomin_d(inst),
                    // (0b011, 0b10100) => self.execute_amomax_d(inst),
                    // (0b011, 0b11000) => self.execute_amominu_d(inst),
                    // (0b011, 0b11100) => self.execute_amomaxu_d(inst),
                    _ => return Err(Exception::IllegalInstruction(inst)),
                }
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b01, 0b100) => {
                // OP
                let shamt = ((self.regs[rs2 as usize] & 0x3f) as u64) as u32;
                match (funct7, funct3) {
                    (0b000_0000, 0b000) => self.execute_add(rd, rs1, rs2),
                    (0b010_0000, 0b000) => self.execute_sub(rd, rs1, rs2),
                    (0b000_0000, 0b001) => self.execute_sll(rd, rs1, shamt),
                    (0b000_0000, 0b010) => self.execute_slt(rd, rs1, rs2),
                    (0b000_0000, 0b011) => self.execute_sltu(rd, rs1, rs2),
                    (0b000_0000, 0b100) => self.execute_xor(rd, rs1, rs2),
                    (0b000_0000, 0b101) => self.execute_srl(rd, rs1, shamt),
                    (0b010_0000, 0b101) => self.execute_sra(rd, rs1, shamt),
                    (0b000_0000, 0b110) => self.execute_or(rd, rs1, rs2),
                    (0b000_0000, 0b111) => self.execute_and(rd, rs1, rs2),
                    (0b000_0001, 0b000) => self.execute_mul(rd, rs1, rs2),
                    // (0b000_0001, 0b001) => self.execute_mulh(),
                    // (0b000_0001, 0b010) => self.execute_mulhsu(),
                    // (0b000_0001, 0b011) => self.execute_mulhu(),
                    // (0b000_0001, 0b100) => self.execute_div(),
                    (0b000_0001, 0b101) => self.execute_divu(rd, rs1, rs2),
                    // (0b000_0001, 0b110) => self.execute_rem(),
                    // (0b000_0001, 0b111) => self.execute_remu(),
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }
            (0b01, 0b101) => {
                // LUI
                self.execute_lui(inst, rd)
            }
            (0b01, 0b110) => {
                // OP-32
                let shamt = (self.regs[rs2 as usize] & 0x1f) as u32;
                match (funct7, funct3) {
                    (0b000_0000, 0b000) => self.execute_addw(rd, rs1, rs2),
                    (0b010_0000, 0b000) => self.execute_subw(rd, rs1, rs2),
                    (0b000_0000, 0b001) => self.execute_sllw(rd, rs1, shamt),
                    (0b000_0000, 0b101) => self.execute_srlw(rd, rs1, shamt),
                    (0b010_0000, 0b101) => self.execute_sraw(rd, rs1, shamt),
                    // (0b000_0001, 0b000) => self.execute_mulw(),
                    // (0b000_0001, 0b100) => self.execute_divw(rd, rs1, rs2),
                    // (0b000_0001, 0b101) => self.execute_divuw(),
                    // (0b000_0001, 0b110) => self.execute_remw(),
                    (0b000_0001, 0b111) => self.execute_remuw(rd, rs1, rs2),
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }

            // Group 2 (inst[6:5] == 10)
            (0b10, 0b000) => {
                // MADD
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b10, 0b001) => {
                // MSUB
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b10, 0b010) => {
                // NMSUB
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b10, 0b011) => {
                // NMADD
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b10, 0b100) => {
                // OP-FP
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b10, 0b110) => {
                // custom-2/rv128
                return Err(Exception::IllegalInstruction(inst));
            }

            // Group 3 (inst[6:5] == 11)
            (0b11, 0b000) => {
                // BRANCH
                // imm[12|10:5|4:1|11] = inst[31|30:25|11:8|7]
                let imm = (((inst & 0x80000000) as i32 as i64 >> 19) as u64)
                                | ((inst & 0x80) << 4) // imm[11]
                                | ((inst >> 20) & 0x7e0) // imm[10:5]
                                | ((inst >> 7) & 0x1e); // imm[4:1]
                match funct3 {
                    0b000 => inst_step = self.execute_beq(rs1, rs2, imm),
                    0b001 => inst_step = self.execute_bne(rs1, rs2, imm),
                    0b100 => inst_step = self.execute_blt(rs1, rs2, imm),
                    0b101 => inst_step = self.execute_bge(rs1, rs2, imm),
                    0b110 => inst_step = self.execute_bltu(rs1, rs2, imm),
                    0b111 => inst_step = self.execute_bgeu(rs1, rs2, imm),
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }
            (0b11, 0b001) => {
                // JALR
                let imm = ((((inst & 0xfff00000) as i32) as i64) >> 20) as u64;
                self.execute_jalr(rd, rs1, imm);
                inst_step = 0;
            }
            (0b11, 0b010) => {
                // reserved
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b11, 0b011) => {
                // JAL
                // imm[20|10:1|11|19:12] = inst[31|30:21|20|19:12]
                let imm = (((inst & 0x80000000) as i32 as i64 >> 11) as u64) // imm[20]
                    | (inst & 0xff000) // imm[19:12]
                    | ((inst >> 9) & 0x800) // imm[11]
                    | ((inst >> 20) & 0x7fe); // imm[10:1]
                self.execute_jal(rd, imm);
                inst_step = 0;
            }
            (0b11, 0b100) => {
                // SYSTEM
                let csr_addr = inst >> 20;
                let uimm = rs1 as u64;
                match (rs2, funct3) {
                    (0b0, 0b000) => self.execute_ecall()?,
                    (0b1, 0b000) => self.execute_ebreak()?,
                    (0b00010, 0b000) => match funct7 {
                        0b0001000 => self.execute_sret(),
                        0b0011000 => self.execute_mret(),
                        // 0b0111000 => self.execute_mnret(),
                        _ => {
                            return Err(Exception::IllegalInstruction(inst));
                        }
                    },
                    (_, 0b000) => match funct7 {
                        // 0b0001000 => self.execute_wfi(),
                        0b0001001 => self.execute_sfence_vma(),
                        // 0b0010001 => self.execute_hfence_vvma(),
                        // 0b0110001 => self.execute_hfence_gvma(),
                        _ => {
                            return Err(Exception::IllegalInstruction(inst));
                        }
                    },
                    (_, 0b001) => self.execute_csrrw(csr_addr, rd, rs1),
                    (_, 0b010) => self.execute_csrrs(csr_addr, rd, rs1),
                    (_, 0b011) => self.execute_csrrc(csr_addr, rd, rs1),
                    (_, 0b101) => self.execute_csrrwi(csr_addr, rd, uimm),
                    (_, 0b110) => self.execute_csrrsi(csr_addr, rd, uimm),
                    (_, 0b111) => self.execute_csrrci(csr_addr, rd, uimm),
                    // (0b01101, 0b000) => self.execute_wrs_nto(),
                    // (0b11101, 0b000) => self.execute_wrs_sto(),
                    _ => {
                        return Err(Exception::IllegalInstruction(inst));
                    }
                }
            }
            (0b11, 0b101) => {
                // OP-VE
                return Err(Exception::IllegalInstruction(inst));
            }
            (0b11, 0b110) => {
                // custom-3/rv128
                return Err(Exception::IllegalInstruction(inst));
            }

            // Default case (any other combination)
            _ => {
                return Err(Exception::IllegalInstruction(inst));
            }
        }

        self.pc = self.pc.wrapping_add(inst_step);
        Ok(())
    }

    pub fn check_interrupt(&mut self) -> Option<Interrupt> {
        // When a hart is executing in privilege mode x, interrupts are
        // globally enabled when xIE=1 and globally disabled when xIE=0.
        // Interrupts for lower-privilege modes, w<x, are always globally
        // disabled regardless of the setting of any global wIE bit for
        // the lower-privilege mode. Interrupts for higher-privilege modes,
        // y>x, are always globally enabled regardless of the setting of
        // the global yIE bit for the higher-privilege mode.
        // Higher-privilege-level code can use separate per-interrupt
        // enable bits to disable selected higher-privilege-mode interrupts
        // before ceding control to a lower-privilege mode.

        // An interrupt i will trap to M-mode (causing the privilege mode
        // to change to M-mode) if all of the following are true: (a) either
        // the current privilege mode is M and the MIE bit in the mstatus
        // register is set, or the current privilege mode has less privilege
        // than M-mode; (b) bit i is set in both mip and mie; and (c) if
        // register mideleg exists, bit i is not set in mideleg.

        // First, check if the hart is in Machine mode and if interrupts are enabled (MIE = 1)
        if self.mode == Mode::Machine && (self.csr.load(MSTATUS) & (1 << 3) == 0) {
            // If MIE is 0, no interrupts should be handled in Machine mode
            return None;
        }

        // let irq = self.bus.plic.borrow_mut().claim();

        // Check for any pending interrupts in Machine mode
        // MIE enables interrupts for Machine mode, and MIP holds the pending interrupts for M-mode
        let machine_pending = self.csr.load(MIE) & self.csr.load(MIP);

        if (machine_pending & 1 << 11) != 0 {
            self.csr.store(MIP, self.csr.load(MIP) & !(1 << 11));
            return Some(Interrupt::MachineExternalInterrupt);
        }
        if (machine_pending & 1 << 3) != 0 {
            self.csr.store(MIP, self.csr.load(MIP) & !(1 << 3));
            return Some(Interrupt::MachineSoftwareInterrupt);
        }
        if (machine_pending & 1 << 7) != 0 {
            self.csr.store(MIP, self.csr.load(MIP) & !(1 << 7));
            return Some(Interrupt::MachineTimerInterrupt);
        }

        // If still in Machine mode and no interrupt was handled, return None
        // Machine mode cannot handle Supervisor interrupts, so exit early
        if self.mode == Mode::Machine {
            return None;
        }

        // In Supervisor mode, check if Supervisor interrupts are enabled (SIE = 1)
        if self.mode == Mode::Supervisor && (self.csr.load(SSTATUS) & (1 << 1) == 0) {
            // If SIE is 0, no Supervisor interrupts will be handled
            return None;
        }

        // Now in Supervisor mode, and Supervisor interrupts are enabled (SIE = 1)
        // Check for pending Supervisor interrupts by examining SIP (pending) and SIE (enabled)
        let supervisor_pending = self.csr.load(SIE) & self.csr.load(SIP);

        if (supervisor_pending & 1 << 9) != 0 {
            self.csr.store(SIP, self.csr.load(SIP) & !(1 << 9));
            return Some(Interrupt::SupervisorExternalInterrupt);
        }
        if (supervisor_pending & 1 << 1) != 0 {
            self.csr.store(SIP, self.csr.load(SIP) & !(1 << 1));
            return Some(Interrupt::SupervisorSoftwareInterrupt);
        }
        if (supervisor_pending & 1 << 5) != 0 {
            self.csr.store(SIP, self.csr.load(SIP) & !(1 << 5));
            return Some(Interrupt::SupervisorTimerInterrupt);
        }
        // If no interrupt, return None
        return None;
    }

    pub fn handle_exception(&mut self, exception: Exception) {
        let exception_pc = self.pc;
        let prev_mode = self.mode;
        let cause = exception.code();

        let is_user_or_supervisor = prev_mode != Mode::Machine;
        let medeleg = self.csr.load(MEDELEG);
        let is_exception_delegated = (medeleg.wrapping_shr(cause as u32) & 1) != 0;
        if is_user_or_supervisor && is_exception_delegated {
            self.mode = Mode::Supervisor;
        }
        // 1.
        // The mtvec register must always be implemented, but can contain a
        // read-only value. If mtvec is writable, the set of values the register
        // may hold can vary by implementation. The value in the BASE field
        // must always be aligned on a 4-byte boundary, and the MODE setting
        // may impose additional alignment constraints on the value in the
        // BASE field.
        // The mtvec register is an MXLEN-bit WARL read/write register that
        // holds trap vector configuration, consisting of a vector base
        // address (BASE) and a vector mode (MODE).
        // When MODE=Direct, all traps into machine mode cause the pc to be
        // set to the address in the BASE field. When MODE=Vectored, all
        // synchronous exceptions into machine mode cause the pc to be set
        // to the address in the BASE field, whereas interrupts cause the pc
        // to be set to the address in the BASE field plus four times the
        // interrupt cause number.
        // 2.
        // When a trap is taken into M-mode, mepc is written with the
        // virtual address of the instruction that was interrupted or that
        // encountered the exception. Otherwise, mepc is never written by the
        // implementation, though it may be explicitly written by software
        // 3.
        // When a trap is taken into M-mode, mcause is written with a code
        // indicating the event that caused the trap. Otherwise, mcause is
        // never written by the implementation, though it may be explicitly
        // written by software.
        // 4.
        // When a trap is taken into M-mode, mtval is either set to zero or
        // written with exception-specific information to assist software in
        // handling the trap. Otherwise, mtval is never written by the
        // implementation, though it may be explicitly written by software.
        // 5.
        // When a trap is taken from privilege mode y into privilege mode x,
        // xPIE is set to the value of xIE; xIE is set to 0; and xPP is set to y.
        match self.mode {
            Mode::Machine => {
                let mtvec = self.csr.load(MTVEC);
                let base = mtvec & !0b11;
                let _mode = mtvec & 0b11;
                // When MODE=Direct, all traps into machine mode cause the pc to be
                // set to the address in the BASE field. When MODE=Vectored, all
                // synchronous exceptions into machine mode cause the pc to be set
                // to the address in the BASE field, whereas interrupts cause the pc
                // to be set to the address in the BASE field plus four times the
                // interrupt cause number.
                self.pc = base;
                self.csr.store(MEPC, exception_pc as u64);
                self.csr.store(MCAUSE, cause);
                self.csr.store(MTVAL, exception.code());
                let mut mstatus = self.csr.load(MSTATUS);
                let mie = (mstatus >> 3) & 0b1;
                // set xIE = 0
                mstatus &= !(1 << 3);
                // set xPIE = previous xIE
                mstatus &= !(1 << 7);
                mstatus |= mie << 7;
                // set xPP = previous mode
                mstatus &= !(0b11 << 11);
                mstatus |= prev_mode.code() << 11;
                self.csr.store(MSTATUS, mstatus);
            }
            Mode::Supervisor => {
                let stvec = self.csr.load(STVEC);
                let base = stvec & !0b11;
                let _mode = stvec & 0b11;
                self.pc = base;
                self.csr.store(SEPC, exception_pc as u64);
                self.csr.store(SCAUSE, cause);
                self.csr.store(STVAL, exception.code());
                let mut sstatus = self.csr.load(SSTATUS);
                let sie = (sstatus >> 1) & 0b1;
                // set xIE = 0
                sstatus &= !(1 << 1);
                // set xPIE = previous xIE
                sstatus &= !(1 << 5);
                sstatus |= sie << 5;
                // set xPP = previous mode
                sstatus &= !(0b1 << 8);
                sstatus |= prev_mode.code() << 8;
                self.csr.store(SSTATUS, sstatus);
            }
            _ => {}
        }
    }

    pub fn handle_interrupt(&mut self, interrupt: Interrupt) {
        let interrupt_pc = self.pc;
        let prev_mode = self.mode;
        let cause = interrupt.code();
        let cause_code = cause & !INTERRUPT_BIT;

        let is_user_or_supervisor = prev_mode != Mode::Machine;
        let mideleg = self.csr.load(MIDELEG);
        let is_interrupt_delegated = (mideleg.wrapping_shr(cause_code as u32) & 1) != 0;
        if is_user_or_supervisor && is_interrupt_delegated {
            self.mode = Mode::Supervisor;
        }
        match self.mode {
            Mode::Machine => {
                let mtvec = self.csr.load(MTVEC);
                let base = mtvec & !0b11;
                let mode = mtvec & 0b11;
                // When MODE=Direct, all traps into machine mode cause the pc to be
                // set to the address in the BASE field. When MODE=Vectored, all
                // synchronous exceptions into machine mode cause the pc to be set
                // to the address in the BASE field, whereas interrupts cause the pc
                // to be set to the address in the BASE field plus four times the
                // interrupt cause number.
                self.pc = match mode {
                    // Direct
                    0b00 => base,
                    // Vectored
                    0b01 => base + cause_code << 2,
                    _ => unreachable!(),
                };
                self.csr.store(MEPC, interrupt_pc as u64);
                self.csr.store(MCAUSE, cause);
                // When a trap is taken into M-mode, mtval is either set to zero or
                // written with exception-specific information to assist software in
                // handling the trap.
                self.csr.store(MTVAL, 0);
                let mut mstatus = self.csr.load(MSTATUS);
                let mie = (mstatus >> 3) & 0b1;
                // set xIE = 0
                mstatus &= !(1 << 3);
                // set xPIE = previous xIE
                mstatus &= !(1 << 7);
                mstatus |= mie << 7;
                // set xPP = previous mode
                mstatus &= !(0b11 << 11);
                mstatus |= prev_mode.code() << 11;
                self.csr.store(MSTATUS, mstatus);
            }
            Mode::Supervisor => {
                let stvec = self.csr.load(STVEC);
                let base = stvec & !0b11;
                let mode = stvec & 0b11;
                self.pc = match mode {
                    // Direct
                    0b00 => base,
                    // Vectored
                    0b01 => base + cause_code << 2,
                    _ => unreachable!(),
                };
                self.csr.store(SEPC, interrupt_pc as u64);
                self.csr.store(SCAUSE, cause);
                self.csr.store(STVAL, 0);
                let mut sstatus = self.csr.load(SSTATUS);
                let sie = (sstatus >> 1) & 0b1;
                // set xIE = 0
                sstatus &= !(1 << 1);
                // set xPIE = previous xIE
                sstatus &= !(1 << 5);
                sstatus |= sie << 5;
                // set xPP = previous mode
                sstatus &= !(0b1 << 8);
                sstatus |= prev_mode.code() << 8;
                self.csr.store(SSTATUS, sstatus);
            }
            _ => {}
        }
    }
}

#![allow(unused)]

use crate::cpu::Mode;
use crate::lib::address::*;
use crate::{cpu::Cpu, Exception};

impl Cpu {
    #[inline(always)]
    pub fn execute_(&mut self) {}

    #[inline(always)]
    pub fn execute_lui(&mut self, inst: u64, rd: u64) {
        self.regs[rd as usize] = (inst & 0xfffff000) as i32 as i64 as u64;
    }

    #[inline(always)]
    pub fn execute_auipc(&mut self, rd: u64, imm: u64) {
        self.regs[rd as usize] = self.pc.wrapping_add(imm);
    }

    #[inline(always)]
    pub fn execute_jal(&mut self, rd: u64, imm: u64) {
        self.regs[rd as usize] = self.pc.wrapping_add(4);
        self.pc = self.pc.wrapping_add(imm);
    }

    #[inline(always)]
    pub fn execute_jalr(&mut self, rd: u64, rs1: u64, imm: u64) {
        let t = self.pc;
        self.pc = (self.regs[rs1 as usize].wrapping_add(imm)) & !1;
        self.regs[rd as usize] = t.wrapping_add(4);
    }

    #[inline(always)]
    pub fn execute_beq(&mut self, rs1: u64, rs2: u64, imm: u64) -> u64 {
        if self.regs[rs1 as usize] == self.regs[rs2 as usize] {
            self.pc = self.pc.wrapping_add(imm);
            return 0;
        }
        4
    }

    #[inline(always)]
    pub fn execute_bne(&mut self, rs1: u64, rs2: u64, imm: u64) -> u64 {
        if self.regs[rs1 as usize] != self.regs[rs2 as usize] {
            self.pc = self.pc.wrapping_add(imm);
            return 0;
        }
        4
    }

    #[inline(always)]
    pub fn execute_blt(&mut self, rs1: u64, rs2: u64, imm: u64) -> u64 {
        if (self.regs[rs1 as usize] as i64) < (self.regs[rs2 as usize] as i64) {
            self.pc = self.pc.wrapping_add(imm);
            return 0;
        }
        4
    }

    #[inline(always)]
    pub fn execute_bge(&mut self, rs1: u64, rs2: u64, imm: u64) -> u64 {
        if (self.regs[rs1 as usize] as i64) >= (self.regs[rs2 as usize] as i64) {
            self.pc = self.pc.wrapping_add(imm);
            return 0;
        }
        4
    }

    #[inline(always)]
    pub fn execute_bltu(&mut self, rs1: u64, rs2: u64, imm: u64) -> u64 {
        if self.regs[rs1 as usize] < self.regs[rs2 as usize] {
            self.pc = self.pc.wrapping_add(imm);
            return 0;
        }
        4
    }

    #[inline(always)]
    pub fn execute_bgeu(&mut self, rs1: u64, rs2: u64, imm: u64) -> u64 {
        if self.regs[rs1 as usize] >= self.regs[rs2 as usize] {
            self.pc = self.pc.wrapping_add(imm);
            return 0;
        }
        4
    }

    #[inline(always)]
    pub fn execute_lb(&mut self, addr: u64, rd: u64) -> Result<(), Exception> {
        let val = self.load(addr, 8)?;
        self.regs[rd as usize] = val as i8 as i64 as u64;
        Ok(())
    }

    #[inline(always)]
    pub fn execute_lh(&mut self, addr: u64, rd: u64) -> Result<(), Exception> {
        let val = self.load(addr, 16)?;
        self.regs[rd as usize] = val as i16 as i64 as u64;
        Ok(())
    }

    #[inline(always)]
    pub fn execute_lw(&mut self, addr: u64, rd: u64) -> Result<(), Exception> {
        let val = self.load(addr, 32)?;
        self.regs[rd as usize] = val as i32 as i64 as u64;
        Ok(())
    }

    #[inline(always)]
    pub fn execute_ld(&mut self, addr: u64, rd: u64) -> Result<(), Exception> {
        let val = self.load(addr, 64)?;
        self.regs[rd as usize] = val as u64;
        Ok(())
    }

    #[inline(always)]
    pub fn execute_lbu(&mut self, addr: u64, rd: u64) -> Result<(), Exception> {
        let val = self.load(addr, 8)?;
        self.regs[rd as usize] = val as u64;
        Ok(())
    }

    #[inline(always)]
    pub fn execute_lhu(&mut self, addr: u64, rd: u64) -> Result<(), Exception> {
        let val = self.load(addr, 16)?;
        self.regs[rd as usize] = val as u64;
        Ok(())
    }

    #[inline(always)]
    pub fn execute_lwu(&mut self, addr: u64, rd: u64) -> Result<(), Exception> {
        let val = self.load(addr, 32)?;
        self.regs[rd as usize] = val as u64;
        Ok(())
    }

    #[inline(always)]
    pub fn execute_sb(&mut self, addr: u64, rs2: u64) -> Result<(), Exception> {
        self.store(addr, 8, self.regs[rs2 as usize])?;
        Ok(())
    }

    #[inline(always)]
    pub fn execute_sh(&mut self, addr: u64, rs2: u64) -> Result<(), Exception> {
        self.store(addr, 16, self.regs[rs2 as usize])?;
        Ok(())
    }

    #[inline(always)]
    pub fn execute_sw(&mut self, addr: u64, rs2: u64) -> Result<(), Exception> {
        self.store(addr, 32, self.regs[rs2 as usize])?;
        Ok(())
    }

    #[inline(always)]
    pub fn execute_sd(&mut self, addr: u64, rs2: u64) -> Result<(), Exception> {
        self.store(addr, 64, self.regs[rs2 as usize])?;
        Ok(())
    }

    #[inline(always)]
    pub fn execute_addi(&mut self, rd: u64, rs1: u64, imm: u64) {
        self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_add(imm);
    }

    #[inline(always)]
    pub fn execute_slti(&mut self, rd: u64, rs1: u64, imm: u64) {
        self.regs[rd as usize] = if (self.regs[rs1 as usize] as i64) < (imm as i64) {
            1
        } else {
            0
        };
    }

    #[inline(always)]
    pub fn execute_sltiu(&mut self, rd: u64, rs1: u64, imm: u64) {
        self.regs[rd as usize] = if self.regs[rs1 as usize] < imm { 1 } else { 0 };
    }

    #[inline(always)]
    pub fn execute_xori(&mut self, rd: u64, rs1: u64, imm: u64) {
        self.regs[rd as usize] = self.regs[rs1 as usize] ^ imm;
    }

    #[inline(always)]
    pub fn execute_ori(&mut self, rd: u64, rs1: u64, imm: u64) {
        self.regs[rd as usize] = self.regs[rs1 as usize] | imm;
    }

    #[inline(always)]
    pub fn execute_andi(&mut self, rd: u64, rs1: u64, imm: u64) {
        self.regs[rd as usize] = self.regs[rs1 as usize] & imm;
    }

    #[inline(always)]
    pub fn execute_slli(&mut self, rd: u64, rs1: u64, shamt: u32) {
        self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_shl(shamt);
    }

    #[inline(always)]
    pub fn execute_srli(&mut self, rd: u64, rs1: u64, shamt: u32) {
        self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_shr(shamt);
    }

    #[inline(always)]
    pub fn execute_srai(&mut self, rd: u64, rs1: u64, shamt: u32) {
        self.regs[rd as usize] = (self.regs[rs1 as usize] as i64).wrapping_shr(shamt) as u64;
    }

    #[inline(always)]
    pub fn execute_add(&mut self, rd: u64, rs1: u64, rs2: u64) {
        self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_add(self.regs[rs2 as usize]);
    }

    #[inline(always)]
    pub fn execute_sub(&mut self, rd: u64, rs1: u64, rs2: u64) {
        self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_sub(self.regs[rs2 as usize]);
    }

    #[inline(always)]
    pub fn execute_sll(&mut self, rd: u64, rs1: u64, shamt: u32) {
        self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_shl(shamt);
    }

    #[inline(always)]
    pub fn execute_slt(&mut self, rd: u64, rs1: u64, rs2: u64) {
        self.regs[rd as usize] =
            if (self.regs[rs1 as usize] as i64) < (self.regs[rs2 as usize] as i64) {
                1
            } else {
                0
            };
    }

    #[inline(always)]
    pub fn execute_sltu(&mut self, rd: u64, rs1: u64, rs2: u64) {
        self.regs[rd as usize] = if (self.regs[rs1 as usize]) < (self.regs[rs2 as usize]) {
            1
        } else {
            0
        };
    }

    #[inline(always)]
    pub fn execute_xor(&mut self, rd: u64, rs1: u64, rs2: u64) {
        self.regs[rd as usize] = self.regs[rs1 as usize] ^ self.regs[rs2 as usize];
    }

    #[inline(always)]
    pub fn execute_srl(&mut self, rd: u64, rs1: u64, shamt: u32) {
        self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_shr(shamt);
    }

    #[inline(always)]
    pub fn execute_sra(&mut self, rd: u64, rs1: u64, shamt: u32) {
        self.regs[rd as usize] = (self.regs[rs1 as usize] as i64).wrapping_shr(shamt) as u64;
    }

    #[inline(always)]
    pub fn execute_or(&mut self, rd: u64, rs1: u64, rs2: u64) {
        self.regs[rd as usize] = self.regs[rs1 as usize] | self.regs[rs2 as usize];
    }

    #[inline(always)]
    pub fn execute_and(&mut self, rd: u64, rs1: u64, rs2: u64) {
        self.regs[rd as usize] = self.regs[rs1 as usize] & self.regs[rs2 as usize];
    }

    #[inline(always)]
    pub fn execute_fence(&mut self) {}

    #[inline(always)]
    pub fn execute_fence_tso(&mut self) {}

    #[inline(always)]
    pub fn execute_sfence_vma(&mut self) {
        return;
    }

    #[inline(always)]
    pub fn execute_pause(&mut self) {}

    #[inline(always)]
    pub fn execute_ecall(&mut self) -> Result<(), Exception> {
        match self.mode {
            Mode::User => Err(Exception::EnvironmentCallFromUMode(self.pc)),
            Mode::Supervisor => Err(Exception::EnvironmentCallFromSMode(self.pc)),
            Mode::Machine => Err(Exception::EnvironmentCallFromMMode(self.pc)),
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub fn execute_ebreak(&mut self) -> Result<(), Exception> {
        Err(Exception::Breakpoint(self.pc))
    }

    #[inline(always)]
    pub fn execute_addiw(&mut self, rd: u64, rs1: u64, imm: u64) {
        self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_add(imm) as i32 as i64 as u64;
    }

    #[inline(always)]
    pub fn execute_slliw(&mut self, rd: u64, rs1: u64, shamt: u32) {
        self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_shl(shamt) as i32 as i64 as u64;
    }

    #[inline(always)]
    pub fn execute_srliw(&mut self, rd: u64, rs1: u64, shamt: u32) {
        self.regs[rd as usize] =
            (self.regs[rs1 as usize] as u32).wrapping_shr(shamt) as i32 as i64 as u64;
    }

    #[inline(always)]
    pub fn execute_sraiw(&mut self, rd: u64, rs1: u64, shamt: u32) {
        self.regs[rd as usize] = (self.regs[rs1 as usize] as i32).wrapping_shr(shamt) as i64 as u64;
    }

    #[inline(always)]
    pub fn execute_addw(&mut self, rd: u64, rs1: u64, rs2: u64) {
        self.regs[rd as usize] =
            self.regs[rs1 as usize].wrapping_add(self.regs[rs2 as usize]) as i32 as i64 as u64;
    }

    #[inline(always)]
    pub fn execute_subw(&mut self, rd: u64, rs1: u64, rs2: u64) {
        self.regs[rd as usize] =
            self.regs[rs1 as usize].wrapping_sub(self.regs[rs2 as usize]) as i32 as i64 as u64;
    }

    #[inline(always)]
    pub fn execute_sllw(&mut self, rd: u64, rs1: u64, shamt: u32) {
        self.regs[rd as usize] = (self.regs[rs1 as usize] as u32).wrapping_shl(shamt) as i32 as u64;
    }

    #[inline(always)]
    pub fn execute_srlw(&mut self, rd: u64, rs1: u64, shamt: u32) {
        self.regs[rd as usize] = (self.regs[rs1 as usize] as u32).wrapping_shr(shamt) as i32 as u64;
    }

    #[inline(always)]
    pub fn execute_sraw(&mut self, rd: u64, rs1: u64, shamt: u32) {
        self.regs[rd as usize] = ((self.regs[rs1 as usize] as i32) >> (shamt as i32)) as u64;
    }

    // ---------------------------------------
    // Instruction | rd  | rs1 | Read | Write
    // ---------------------------------------
    // CSRRW       | x0  | -   | no   | yes
    // CSRRW       | !x0 | -   | yes  | yes
    // CSRRS/C     | -   | x0  | yes  | no
    // CSRRS/C     | -   | !x0 | yes  | yes

    #[inline(always)]
    pub fn execute_csrrw(&mut self, csr_addr: u64, rd: u64, rs1: u64) {
        let t = self.csr.load(csr_addr);
        self.csr.store(csr_addr, self.regs[rs1 as usize]);
        self.regs[rd as usize] = if rd == 0 { self.regs[rd as usize] } else { t };
    }

    #[inline(always)]
    pub fn execute_csrrs(&mut self, csr_addr: u64, rd: u64, rs1: u64) {
        let t = self.csr.load(csr_addr);
        self.csr.store(csr_addr, t | self.regs[rs1 as usize]);
        self.regs[rd as usize] = if rd == 0 { self.regs[rd as usize] } else { t };
    }

    #[inline(always)]
    pub fn execute_csrrc(&mut self, csr_addr: u64, rd: u64, rs1: u64) {
        let t = self.csr.load(csr_addr);
        self.csr.store(csr_addr, t & (!self.regs[rs1 as usize]));
        self.regs[rd as usize] = if rd == 0 { self.regs[rd as usize] } else { t };
    }

    // ---------------------------------------
    // Instruction | rd  | uimm | Read | Write
    // ---------------------------------------
    // CSRRWI      | x0  | -    | no   | yes
    // CSRRWI      | !x0 | -    | yes  | yes
    // CSRRS/CI    | -   | 0    | yes  | no
    // CSRRS/CI    | -   | !0   | yes  | yes

    #[inline(always)]
    pub fn execute_csrrwi(&mut self, csr_addr: u64, rd: u64, uimm: u64) {
        let t = self.csr.load(csr_addr);
        self.csr.store(csr_addr, uimm);
        self.regs[rd as usize] = if rd == 0 { self.regs[rd as usize] } else { t };
    }

    #[inline(always)]
    pub fn execute_csrrsi(&mut self, csr_addr: u64, rd: u64, uimm: u64) {
        let t = self.csr.load(csr_addr);
        self.csr.store(csr_addr, t | uimm);
        self.regs[rd as usize] = if rd == 0 { self.regs[rd as usize] } else { t };
    }

    #[inline(always)]
    pub fn execute_csrrci(&mut self, csr_addr: u64, rd: u64, uimm: u64) {
        let t = self.csr.load(csr_addr);
        self.csr.store(csr_addr, t & (!uimm));
        self.regs[rd as usize] = if rd == 0 { self.regs[rd as usize] } else { t };
    }

    #[inline(always)]
    pub fn execute_mul(&mut self, rd: u64, rs1: u64, rs2: u64) {
        self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_mul(self.regs[rs2 as usize]);
    }

    #[inline(always)]
    pub fn execute_mulh(&mut self) {}

    #[inline(always)]
    pub fn execute_mulhsu(&mut self) {}

    #[inline(always)]
    pub fn execute_mulhu(&mut self) {}

    #[inline(always)]
    pub fn execute_div(&mut self) {}

    #[inline(always)]
    pub fn execute_divu(&mut self, rd: u64, rs1: u64, rs2: u64) {
        let divisor = self.regs[rs2 as usize];
        self.regs[rd as usize] = match divisor {
            0 => 0xffffffff_ffffffff,
            _ => {
                let dividend = self.regs[rs1 as usize];
                dividend.wrapping_div(divisor)
            }
        };
    }

    #[inline(always)]
    pub fn execute_rem(&mut self) {}

    #[inline(always)]
    pub fn execute_remu(&mut self) {}

    #[inline(always)]
    pub fn execute_mulw(&mut self) {}

    #[inline(always)]
    pub fn execute_divw(&mut self) {}

    #[inline(always)]
    pub fn execute_divuw(&mut self) {}

    #[inline(always)]
    pub fn execute_remw(&mut self) {}

    #[inline(always)]
    pub fn execute_remuw(&mut self, rd: u64, rs1: u64, rs2: u64) {
        let divisor = self.regs[rs2 as usize] as u32;
        self.regs[rd as usize] = match divisor {
            0 => self.regs[rs1 as usize],
            _ => {
                let dividend = self.regs[rs1 as usize] as u32;
                dividend.wrapping_rem(divisor) as i32 as u64
            }
        };
    }

    #[inline(always)]
    pub fn execute_lr_w(&mut self) {}

    #[inline(always)]
    pub fn execute_sc_w(&mut self) {}

    #[inline(always)]
    pub fn execute_amoswap_w(&mut self, rd: u64, rs1: u64, rs2: u64) -> Result<(), Exception> {
        let t = self.load(self.regs[rs1 as usize], 32)?;
        self.store(self.regs[rs1 as usize], 32, self.regs[rs2 as usize])?;
        self.regs[rd as usize] = t;
        Ok(())
    }

    #[inline(always)]
    pub fn execute_amoadd_w(&mut self, rd: u64, rs1: u64, rs2: u64) -> Result<(), Exception> {
        let t = self.load(self.regs[rs1 as usize], 32)?;
        self.store(
            self.regs[rs1 as usize],
            32,
            t.wrapping_add(self.regs[rs2 as usize]),
        )?;
        self.regs[rd as usize] = t;
        Ok(())
    }

    #[inline(always)]
    pub fn execute_amoxor_w(&mut self) {}

    #[inline(always)]
    pub fn execute_amoand_w(&mut self) {}

    #[inline(always)]
    pub fn execute_amoor_w(&mut self) {}

    #[inline(always)]
    pub fn execute_amomin_w(&mut self) {}

    #[inline(always)]
    pub fn execute_amomax_w(&mut self) {}

    #[inline(always)]
    pub fn execute_amominu_w(&mut self) {}

    #[inline(always)]
    pub fn execute_amomaxu_w(&mut self) {}

    #[inline(always)]
    pub fn execute_lr_d(&mut self) {}

    #[inline(always)]
    pub fn execute_sc_d(&mut self) {}

    #[inline(always)]
    pub fn execute_amoswap_d(&mut self, rd: u64, rs1: u64, rs2: u64) -> Result<(), Exception> {
        let t = self.load(self.regs[rs1 as usize], 64)?;
        self.store(self.regs[rs1 as usize], 64, self.regs[rs2 as usize])?;
        self.regs[rd as usize] = t;
        Ok(())
    }

    #[inline(always)]
    pub fn execute_amoadd_d(&mut self, rd: u64, rs1: u64, rs2: u64) -> Result<(), Exception> {
        let t = self.load(self.regs[rs1 as usize], 64)?;
        self.store(
            self.regs[rs1 as usize],
            64,
            t.wrapping_add(self.regs[rs2 as usize]),
        )?;
        self.regs[rd as usize] = t;
        Ok(())
    }

    #[inline(always)]
    pub fn execute_amoxor_d(&mut self) {}

    #[inline(always)]
    pub fn execute_amoand_d(&mut self) {}

    #[inline(always)]
    pub fn execute_amoor_d(&mut self) {}

    #[inline(always)]
    pub fn execute_amomin_d(&mut self) {}

    #[inline(always)]
    pub fn execute_amomax_d(&mut self) {}

    #[inline(always)]
    pub fn execute_amominu_d(&mut self) {}

    #[inline(always)]
    pub fn execute_amomaxu_d(&mut self) {}

    pub fn execute_sret(&mut self) {
        // An MRET or SRET instruction is used to return from a
        // trap in M-mode or S-mode respectively. When
        // executing an xRET instruction, supposing xPP holds
        // the value y, xIE is set to xPIE; the privilege mode is
        // changed to y; xPIE is set to 1; and xPP is set to the
        // least-privileged supported mode (U if U-mode is
        // implemented, else M). If y≠M, xRET also sets MPRV=0.
        let mut sstatus = self.csr.load(SSTATUS);
        // When an SRET instruction is executed to return from the trap handler,
        // the privilege level is set to user mode if the SPP bit is 0,
        // or supervisor mode if the SPP bit is 1; SPP is then set to 0.
        self.mode = match (sstatus >> 8) & 0b1 {
            0 => Mode::User,
            _ => Mode::Supervisor,
        };
        // SET MPRV to 0
        let mut mstatus = self.csr.load(MSTATUS);
        mstatus &= !(1 << 17);
        self.csr.store(MSTATUS, mstatus);
        // set SPP to 0
        sstatus &= !(1 << 8);
        // When an SRET instruction is executed,
        // SIE is set to SPIE, then SPIE is set to 1.
        let spie = (sstatus >> 5) & 0b1;
        // clear SIE
        sstatus &= !(1 << 1);
        // set SIE to SPIE
        sstatus |= spie << 1;
        // set SPIE to 1
        sstatus |= 1 << 5;
        self.csr.store(SSTATUS, sstatus);
        // update program counter
        self.pc = self.csr.load(SEPC);
    }

    pub fn execute_mret(&mut self) {
        // An MRET or SRET instruction is used to return from a
        // trap in M-mode or S-mode respectively. When
        // executing an xRET instruction, supposing xPP holds
        // the value y, xIE is set to xPIE; the privilege mode is
        // changed to y; xPIE is set to 1; and xPP is set to the
        // least-privileged supported mode (U if U-mode is
        // implemented, else M). If y≠M, xRET also sets MPRV=0.
        let mut mstatus = self.csr.load(MSTATUS);
        let mpie = (mstatus >> 7) & 0b1;
        // clear MIE
        mstatus &= !(1 << 3);
        // set MIE to MPIE
        mstatus |= mpie << 3;
        self.mode = match (mstatus >> 11) & 0b11 {
            0b11 => Mode::Machine,
            0b01 => {
                mstatus &= !(1 << 17);
                Mode::Supervisor
            }
            _ => {
                mstatus &= !(1 << 17);
                Mode::User
            }
        };
        // set MPIE to 1
        mstatus |= 1 << 7;
        // set MPP to least-privileged supported mode (U: 0b00)
        mstatus &= !(0b11) << 11;
        self.csr.store(MSTATUS, mstatus);
        // update program counter
        self.pc = self.csr.load(MEPC);
    }
}

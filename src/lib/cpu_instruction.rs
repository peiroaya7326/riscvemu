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
    pub fn execute_beq(&mut self, rs1: u64, rs2: u64, imm: u64) -> usize {
        if self.regs[rs1 as usize] == self.regs[rs2 as usize] {
            self.pc = self.pc.wrapping_add(imm);
            return 0;
        }
        4
    }

    #[inline(always)]
    pub fn execute_bne(&mut self, rs1: u64, rs2: u64, imm: u64) -> usize {
        if self.regs[rs1 as usize] != self.regs[rs2 as usize] {
            self.pc = self.pc.wrapping_add(imm);
            return 0;
        }
        4
    }

    #[inline(always)]
    pub fn execute_blt(&mut self, rs1: u64, rs2: u64, imm: u64) -> usize {
        if (self.regs[rs1 as usize] as i64) < (self.regs[rs2 as usize] as i64) {
            self.pc = self.pc.wrapping_add(imm);
            return 0;
        }
        4
    }

    #[inline(always)]
    pub fn execute_bge(&mut self, rs1: u64, rs2: u64, imm: u64) -> usize {
        if (self.regs[rs1 as usize] as i64) >= (self.regs[rs2 as usize] as i64) {
            self.pc = self.pc.wrapping_add(imm);
            return 0;
        }
        4
    }

    #[inline(always)]
    pub fn execute_bltu(&mut self, rs1: u64, rs2: u64, imm: u64) -> usize {
        if self.regs[rs1 as usize] < self.regs[rs2 as usize] {
            self.pc = self.pc.wrapping_add(imm);
            return 0;
        }
        4
    }

    #[inline(always)]
    pub fn execute_bgeu(&mut self, rs1: u64, rs2: u64, imm: u64) -> usize {
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
    pub fn execute_pause(&mut self) {}

    #[inline(always)]
    pub fn execute_ecall(&mut self) {}

    #[inline(always)]
    pub fn execute_ebreak(&mut self) {}

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
    pub fn execute_divu(&mut self) {}

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
    pub fn execute_remuw(&mut self) {}

    #[inline(always)]
    pub fn execute_lr_w(&mut self) {}

    #[inline(always)]
    pub fn execute_sc_w(&mut self) {}

    #[inline(always)]
    pub fn execute_amoswap_w(&mut self) {}

    #[inline(always)]
    pub fn execute_amoadd_w(&mut self) {}

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
    pub fn execute_amoswap_d(&mut self) {}

    #[inline(always)]
    pub fn execute_amoadd_d(&mut self) {}

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
}

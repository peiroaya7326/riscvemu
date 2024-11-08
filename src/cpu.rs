use crate::bus::*;
use crate::csr::*;
use crate::exception::*;
use crate::lib::address::*;
use crate::lib::cpu_instruction::*;

pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,
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
            bus: Bus::new(binary),
            csr: Csr::new(),
        }
    }
    /// Load a value from a dram.
    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        self.bus.load(addr, size)
    }

    /// Store a value to a dram.
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        self.bus.store(addr, size, value)
    }

    pub fn fetch(&self) -> Result<u64, Exception> {
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

        let mut inst_step: usize = 4;
        if op1 == 0b10 {
            inst_step = 2;
        } else if op1 == 0b11 {
            inst_step = 4;
        } else {
            return Err(Exception::IllegalInstruction(inst));
        }

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
                    // (0b000_0001, 0b101) => self.execute_divu(),
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
                    // (0b000_0001, 0b100) => self.execute_divw(),
                    // (0b000_0001, 0b101) => self.execute_divuw(),
                    // (0b000_0001, 0b110) => self.execute_remw(),
                    // (0b000_0001, 0b111) => self.execute_remuw(),
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
                match (rs2, funct3) {
                    // (_, 0b001) => self.execute_csrrw(),
                    // (_, 0b010) => self.execute_csrrs(),
                    // (_, 0b011) => self.execute_csrrc(),
                    // (_, 0b101) => self.execute_csrrwi(),
                    // (_, 0b110) => self.execute_csrrsi(),
                    // (_, 0b111) => self.execute_csrrci(),
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

        self.pc = self.pc.wrapping_add(inst_step as u64);
        Ok(())
    }

    pub fn handle_exception(&self, exception: Exception) {}

    pub fn handle_interrupt(&self) {}
}

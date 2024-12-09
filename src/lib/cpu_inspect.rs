use crate::cpu::Cpu;
use crate::lib::address::*;

impl Cpu {
    pub fn print_registers(&self) {
        let register_names = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];

        for i in 0..8 {
            let idx1 = i * 4;
            let idx2 = i * 4 + 1;
            let idx3 = i * 4 + 2;
            let idx4 = i * 4 + 3;

            if idx4 < self.regs.len() {
                println!(
                    "{:<3} | {:<5} | {:<18} |{:<3} | {:<5} | {:<18} |{:<3} | {:<5} | {:<18} |{:<3} | {:<5} | {:<18}",
                    idx1,
                    register_names[idx1],
                    self.regs[idx1],
                    idx2,
                    register_names[idx2],
                    self.regs[idx2],
                    idx3,
                    register_names[idx3],
                    self.regs[idx3],
                    idx4,
                    register_names[idx4],
                    self.regs[idx4]
                );
            }
        }
        let csr_output = format!(
            "{}\n{}\n",
            format!(
                "mstatus = {:<#18x}  mtvec = {:<#18x}  mepc = {:<#18x}  mcause = {:<#18x}",
                self.csr_load(MSTATUS),
                self.csr_load(MTVEC),
                self.csr_load(MEPC),
                self.csr_load(MCAUSE),
            ),
            format!(
                "sstatus = {:<#18x}  stvec = {:<#18x}  sepc = {:<#18x}  scause = {:<#18x}",
                self.csr_load(SSTATUS),
                self.csr_load(STVEC),
                self.csr_load(SEPC),
                self.csr_load(SCAUSE),
            ),
        );
        println!("{}", csr_output);
    }
}

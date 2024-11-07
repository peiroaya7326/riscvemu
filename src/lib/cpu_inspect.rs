use crate::cpu::Cpu;

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
    }
}

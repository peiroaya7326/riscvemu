use crate::lib::address::*;

const NUM_CSRS: usize = 4096;

pub struct Csr {
    csrs: [u64; NUM_CSRS],
}

impl Csr {
    pub fn new() -> Csr {
        Self {
            csrs: [0; NUM_CSRS],
        }
    }

    pub fn load(&self, addr: u64) -> u64 {
        return self.csrs[addr as usize];
    }

    pub fn store(&mut self, addr: u64, value: u64) {
        self.csrs[addr as usize] = value;
    }
}

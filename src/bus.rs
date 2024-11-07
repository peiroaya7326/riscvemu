use crate::dram::*;
use crate::exception::*;
use crate::lib::address::*;

pub struct Bus {
    pub dram: Dram,
}

impl Bus {
    pub fn new(binary: Vec<u8>) -> Self {
        Self {
            dram: Dram::new(binary),
        }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        if DRAM_BASE <= addr {
            return self.dram.load(addr, size);
        }
        Err(Exception::LoadAccessFault(addr))
    }
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if DRAM_BASE <= addr {
            return self.dram.store(addr, size, value);
        }
        Err(Exception::StoreAMOAccessFault(addr))
    }
}

use crate::exception::*;
use crate::lib::address::*;

pub struct Dram {
    pub dram: Vec<u8>,
}

impl Dram {
    pub fn new(binary: Vec<u8>) -> Self {
        let mut dram = vec![0; DRAM_SIZE as usize];
        dram.splice(..binary.len(), binary.iter().cloned());

        Self { dram }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        if size % 8 != 0 {
            return Err(Exception::LoadAccessFault(addr));
        }
        let index = (addr - DRAM_BASE) as usize;
        let mut value = self.dram[index] as u64;
        for i in 1..size / 8 {
            value |= (self.dram[index + i as usize] as u64) << (i * 8);
        }
        Ok(value)
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if size % 8 != 0 {
            return Err(Exception::StoreAMOAccessFault(addr));
        }
        let index = (addr - DRAM_BASE) as usize;
        println!("{}", index);
        for i in 0..size / 8 {
            self.dram[index + i as usize] = (value >> (i * 8) & 0xff) as u8;
        }
        Ok(())
    }
}

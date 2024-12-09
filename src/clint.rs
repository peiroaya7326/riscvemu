use crate::csr::Csr;
use crate::lib::address::*;
use crate::Exception;
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec;

pub const MSIP_BASE: u64 = CLINT_BASE;
pub const MTIMECMP_BASE: u64 = CLINT_BASE + 0x4000;
pub const MTIME: u64 = CLINT_BASE + 0xbff8;
pub const MTIME_END: u64 = MTIME + 0x8;

pub const MAX_MSIP: usize = 4096;
pub const MAX_MTIMECMP: usize = 4095;

pub struct Timer {
    begin: u64,
    freq: u64,
}

pub struct Clint {
    msip: Vec<u32>,
    mtimecmp: Vec<u64>,
    mtime: Timer,
}

impl Timer {
    pub fn new(freq: u64) -> Self {
        assert!(freq > 0, "Frequency must be greater than zero");
        let mut timer = Self { begin: 0, freq };
        timer.rebase(0);
        timer
    }

    pub fn current_time(&self) -> u64 {
        fn mult_frac(x: u64, n: u64, d: u64) -> u64 {
            // x = qd + r
            let q = x / d;
            let r = x % d;
            // x * n / d
            q * n + r * n / d
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System clock is incorrect or unavailable.");
        let secs = now.as_secs();
        let nanos = now.subsec_nanos() as u64;

        secs * self.freq + mult_frac(nanos, self.freq, 1_000_000_000)
    }

    pub fn get(&self) -> u64 {
        self.current_time() - self.begin
    }

    pub fn rebase(&mut self, time: u64) {
        self.begin = self.current_time() - time;
    }
}

impl Clint {
    pub fn new(timer_freq: u64) -> Self {
        Clint {
            msip: vec![0u32; MAX_MSIP],
            mtimecmp: vec![0u64; MAX_MTIMECMP],
            mtime: Timer::new(timer_freq),
        }
    }

    pub fn check_interrupts(&mut self, hart_id: u64) -> (bool, bool) {
        //
        let time_elapse = self.mtimecmp[hart_id as usize] - self.mtime.get();
        let (mut stip, mut ssip) = (false, false);
        // STIP
        if time_elapse <= 0 {
            stip = true;
        }
        // SSIP
        if self.msip[hart_id as usize] > 0 {
            ssip = true;
        }
        return (stip, ssip);
    }

    pub fn load(&self, addr: u64) -> Result<u64, Exception> {
        match addr {
            MSIP_BASE..MTIMECMP_BASE => {
                let hart_id = ((addr - MSIP_BASE) >> 2) as usize;
                Ok(self.msip[hart_id] as u64)
            }
            MTIMECMP_BASE..MTIME => {
                let hart_id = ((addr - MSIP_BASE) >> 3) as usize;
                Ok(self.mtimecmp[hart_id])
            }
            MTIME..MTIME_END => {
                let time = self.mtime.get();
                // 0xbff8 (low) or 0xbffc (high)
                let value = (time >> (32 & -((((addr - CLINT_BASE) & 0b100) > 0) as i64) as u64))
                    as u32 as u64;
                Ok(value)
            }
            _ => Err(Exception::LoadAccessFault(addr)),
        }
    }

    pub fn store(&mut self, addr: u64, value: u64) -> Result<(), Exception> {
        match addr {
            MSIP_BASE..MTIMECMP_BASE => {
                let hart_id = ((addr - MSIP_BASE) >> 2) as usize;
                self.msip[hart_id] = value as u32;
                Ok(())
            }
            MTIMECMP_BASE..MTIME => {
                let addr = addr - MTIMECMP_BASE;
                let hart_id = (addr >> 3) as usize;
                let mut upper = self.mtimecmp[hart_id] >> 32;
                let mut lower = self.mtimecmp[hart_id] & 0xffff_ffff;
                if (addr & 0b100) > 0 {
                    upper = value;
                } else {
                    lower = value;
                }
                self.mtimecmp[hart_id] = (upper << 32) | lower;
                Ok(())
            }
            MTIME..MTIME_END => {
                let addr = addr - MTIMECMP_BASE;
                let mut upper = self.mtime.begin >> 32;
                let mut lower = self.mtime.begin & 0xffff_ffff;
                if (addr & 0b100) > 0 {
                    upper = value;
                } else {
                    lower = value;
                }
                self.mtime.rebase((upper << 32) | lower);
                Ok(())
            }
            _ => Err(Exception::StoreAMOAccessFault(addr)),
        }
    }
}

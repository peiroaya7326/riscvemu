#[derive(Debug, Copy, Clone)]
pub enum Exception {
    InstructionAddressMisaligned(u64),
    InstructionAccessFault(u64),
    IllegalInstruction(u64),
    Breakpoint(u64),
    LoadAddressMisaligned(u64),
    LoadAccessFault(u64),
    StoreAMOAddressMisaligned(u64),
    StoreAMOAccessFault(u64),
    EnvironmentCallFromUMode(u64),
    EnvironmentCallFromSMode(u64),
    EnvironmentCallFromMMode(u64),
    InstructionPageFault(u64),
    LoadPageFault(u64),
    StoreAMOPageFault(u64),
    SoftwareCheck(u64),
    HardwareError(u64),
}

impl Exception {
    pub const fn value(self) -> u64 {
        match self {
            Exception::InstructionAddressMisaligned(addr) => addr,
            Exception::InstructionAccessFault(addr) => addr,
            Exception::IllegalInstruction(inst) => inst,
            Exception::Breakpoint(pc) => pc,
            Exception::LoadAddressMisaligned(addr) => addr,
            Exception::LoadAccessFault(addr) => addr,
            Exception::StoreAMOAddressMisaligned(addr) => addr,
            Exception::StoreAMOAccessFault(addr) => addr,
            Exception::EnvironmentCallFromUMode(pc) => pc,
            Exception::EnvironmentCallFromSMode(pc) => pc,
            Exception::EnvironmentCallFromMMode(pc) => pc,
            Exception::InstructionPageFault(addr) => addr,
            Exception::LoadPageFault(addr) => addr,
            Exception::StoreAMOPageFault(addr) => addr,
            Exception::SoftwareCheck(val) => val,
            Exception::HardwareError(val) => val,
        }
    }

    pub const fn code(self) -> u64 {
        match self {
            Exception::InstructionAddressMisaligned(_) => 0,
            Exception::InstructionAccessFault(_) => 1,
            Exception::IllegalInstruction(_) => 2,
            Exception::Breakpoint(_) => 3,
            Exception::LoadAddressMisaligned(_) => 4,
            Exception::LoadAccessFault(_) => 5,
            Exception::StoreAMOAddressMisaligned(_) => 6,
            Exception::StoreAMOAccessFault(_) => 7,
            Exception::EnvironmentCallFromUMode(_) => 8,
            Exception::EnvironmentCallFromSMode(_) => 9,
            Exception::EnvironmentCallFromMMode(_) => 11,
            Exception::InstructionPageFault(_) => 12,
            Exception::LoadPageFault(_) => 13,
            Exception::StoreAMOPageFault(_) => 15,
            Exception::SoftwareCheck(_) => 18,
            Exception::HardwareError(_) => 19,
        }
    }

    pub fn is_fatal(self) -> bool {
        match self {
            Exception::InstructionAddressMisaligned(_)
            | Exception::InstructionAccessFault(_)
            | Exception::LoadAccessFault(_)
            | Exception::StoreAMOAddressMisaligned(_)
            | Exception::StoreAMOAccessFault(_)
            | Exception::IllegalInstruction(_) => true,
            _else => false,
        }
    }
}

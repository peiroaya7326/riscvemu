pub const INTERRUPT_BIT: u64 = 1 << 63;

#[derive(Debug, Copy, Clone)]
pub enum Interrupt {
    SupervisorSoftwareInterrupt,
    MachineSoftwareInterrupt,
    SupervisorTimerInterrupt,
    MachineTimerInterrupt,
    SupervisorExternalInterrupt,
    MachineExternalInterrupt,
    CounterOverflowInterrupt,
}

impl Interrupt {
    pub fn code(&self) -> u64 {
        match self {
            Interrupt::SupervisorSoftwareInterrupt => 1 | INTERRUPT_BIT,
            Interrupt::MachineSoftwareInterrupt => 3 | INTERRUPT_BIT,
            Interrupt::SupervisorTimerInterrupt => 5 | INTERRUPT_BIT,
            Interrupt::MachineTimerInterrupt => 7 | INTERRUPT_BIT,
            Interrupt::SupervisorExternalInterrupt => 9 | INTERRUPT_BIT,
            Interrupt::MachineExternalInterrupt => 11 | INTERRUPT_BIT,
            Interrupt::CounterOverflowInterrupt => 13 | INTERRUPT_BIT,
        }
    }
}

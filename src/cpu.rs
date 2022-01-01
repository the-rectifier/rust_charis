mod ram;
mod control;
mod datapath;

use crate::cpu::ram::Ram;
use crate::cpu::control::Control;
use crate::cpu::datapath::Datapath;

pub struct CPU {
    clock: bool,
    clock_cycles: u64,
    control: Control,
    pub datapath: Datapath,
    pub ram: Ram,
}


impl CPU {
    pub fn new() -> Self {
        let cpu = CPU {
            clock: false,
            clock_cycles: 0,
            control: Default::default(),
            datapath: Default::default(),
            ram: Ram::new(),
        };

        cpu
    }
}

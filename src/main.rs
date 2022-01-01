use anyhow::Result;


mod cpu;
use crate::cpu::CPU;

fn main() -> Result<()> {
    
    let mut cpu = CPU::new();

    let res = cpu.datapath.alu.do_alu(i32::MAX, 1, 0b0000)?;

    println!("{}", res);
    println!("{:?}", cpu.datapath.alu);
    
    Ok(())
}

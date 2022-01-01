use anyhow::Result;
use std::num::Wrapping as W;

#[derive(Default)]
pub struct Datapath {
    pub alu: ALU,
    memory_controller: MemoryController,
    fetcher: Fetcher,
    registers: Registers,


    mem_data_reg: InterRegister,
    alu_data_reg: InterRegister,
    inst_data_reg: InterRegister,
    reg_a_data_reg: InterRegister,
    reg_b_data_reg: InterRegister,

}

#[derive(Default)]
struct MemoryController {

}

#[derive(Default)]
struct Fetcher {

}

#[derive(Default)]
struct Registers {
    registers: [u32; 32],
}

#[derive(Default)]
struct InterRegister {
    we: bool,
    data: u32
}

#[derive(Default, Debug)]
pub struct ALU {
    z: bool,
    ovf: bool,
    cout: bool,
}

impl ALU {
    pub fn do_alu(&mut self, a: i32, b: i32, op: u8) -> Result<i32> {
        let mut tmp_out: i32 = 0;

        let add_res = W(a) + W(b);
        let sub_res = W(a) - W(b);

        match op {
            0b0000 => match a.checked_add(b) {
                Some(x) => tmp_out = x,
                None => {
                    tmp_out = format!("{}", add_res).parse::<i32>()?;
                    self.ovf = true;
                }
            },
            0b0001 => match a.checked_sub(b) {
                Some(x) => tmp_out = x,
                None => {
                    tmp_out = format!("{}", sub_res).parse::<i32>()?;
                    self.ovf = true;
                }
            },
            0b0010 => tmp_out = a & b,
            0b0011 => tmp_out = a | b,
            0b0100 => tmp_out = !a,
            0b0101 => tmp_out = !(a & b),
            0b0110 => tmp_out = !(a | b),
            0b1000 => tmp_out = a >> 1,
            0b1001 => tmp_out = ((a as u32) >> 1) as i32,
            0b1010 => tmp_out = ((a as u32) << 1) as i32,
            0b1100 => tmp_out = a.rotate_left(1),
            0b1101 => tmp_out = a.rotate_right(1),
            _ => ()
        };


        if add_res == W(i32::MAX) + W(1) || sub_res == W(i32::MAX) - W(1) {
            self.cout = true;
        }

        if !(op == 0b0000) && !(op == 0b00001) {
            self.cout = false;
        }

        if tmp_out == 0 {
            self.z = true;
        }


        Ok(tmp_out)
    }
}
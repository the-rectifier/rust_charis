pub struct CPU {
    clock_cycles: u64,
    control: Control,
    datapath: Datapath,
    ram: Ram,
}

struct Datapath {
    alu: ALU,
    memory_controller: MemoryController,
    fetcher: Fetcher,
    registers: Registers,


    mem_data_reg: InterRegister,
    alu_data_reg: InterRegister,
    inst_data_reg: InterRegister,
    reg_a_data_reg: InterRegister,
    reg_b_data_reg: InterRegister,

}

struct MemoryController {

}

struct Fetcher {

}

struct Registers {
    registers: [u32; 32],
}

struct InterRegister {
    data: u32
}

struct ALU { 

}

struct Control {

}

struct Ram {
    address: [u32; 2048],
}
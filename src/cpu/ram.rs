use anyhow::Result;

const RAM_LIMIT: usize = 2048;
const INST_LIMIT: usize = 1048;
const DATA_LIMIT: usize = 1048;

pub struct Ram {
    we: bool,
    data: [u8; RAM_LIMIT],
}

impl Ram {
    pub fn new() -> Self {
        let ram = Ram { 
            we: false,
            data: [0; RAM_LIMIT],
        };

        // ram.data[3] = 0xde;
        // ram.data[2] = 0xad;
        // ram.data[1] = 0xbe;
        // ram.data[0] = 0xef;
        ram
    }

    pub fn set_we(&mut self) {
        self.we = true;
    }

    pub fn clear_we(&mut self) {
        self.we = false; 
    }
    
    pub fn get_instruction(&self, addr: u16) -> Result<u32> {
        assert!(addr <= INST_LIMIT as u16);
        let addr = addr as usize;
        let instruction = [self.data[addr], self.data[addr+1], self.data[addr+2], self.data[addr+3]];

        Ok(u32::from_le_bytes(instruction))
    }

    pub fn get_data(&self, addr: u16) -> Result<u32> {
        assert!(addr <= DATA_LIMIT as u16);
        let addr = addr as usize + INST_LIMIT;
        let instruction = [self.data[addr], self.data[addr+1], self.data[addr+2], self.data[addr+3]];

        Ok(u32::from_le_bytes(instruction))
    }

    pub fn write_data(&mut self, addr: u16, data: u32) -> Result<()> {
        assert!(addr <= DATA_LIMIT as u16);
        if !self.we {
            return Ok(());
        }

        let data:[u8; 4] = u32::to_le_bytes(data);
        println!("{:?}", data);
        let addr = addr as usize + INST_LIMIT;
        self.data[addr] = data[0];
        self.data[addr+1] = data[1];
        self.data[addr+2] = data[2];
        self.data[addr+3] = data[3];

        Ok(())
    }
}
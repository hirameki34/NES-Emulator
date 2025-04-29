use super::*;

pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, addr: u16) -> u16 {
        u16::from_le_bytes([self.mem_read(addr), self.mem_read(addr + 1)])
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let data = data.to_le_bytes();
        self.mem_write(addr, data[0]);
        self.mem_write(addr + 1, data[1]);
    }
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}



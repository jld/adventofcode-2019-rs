use std::convert::From;

use crate::{Word, decode::{Insn, Mode, Opcode, DecodeFault}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemMode {
    IRead,
    DRead,
    DWrite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemFault {
    pub addr: Word,
    pub mode: MemMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecError {
    pub pc: Word,
    // Could also include the insn in case of self-modifying code?
    pub fault: ExecFault,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecFault {
    Decode(DecodeFault),
    Mem(MemFault),
    WriteImmediate,
    IO(IOError),
}

impl From<DecodeFault> for ExecFault {
    fn from(inner: DecodeFault) -> Self {
        ExecFault::Decode(inner)
    }
}

impl From<MemFault> for ExecFault {
    fn from(inner: MemFault) -> Self {
        ExecFault::Mem(inner)
    }
}

impl From<IOError> for ExecFault {
    fn from(inner: IOError) -> Self {
        ExecFault::IO(inner)
    }
}

pub struct Computer {
    pc: Word,
    mem: Vec<Word>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stepped {
    Ok,
    Halted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IOError;

pub trait Device {
    fn input(&mut self) -> Result<Word, IOError>;
    fn output(&mut self, val: Word) -> Result<(), IOError>;
}

impl Device for () {
    fn input(&mut self) -> Result<Word, IOError> { Err(IOError) }
    fn output(&mut self, _val: Word) -> Result<(), IOError> { Err(IOError) }
}

impl Computer {
    pub fn new(mem: Vec<Word>) -> Self {
        assert!(mem.len() - 1 <= Word::max_value() as usize);
        Self { pc: 0, mem }
    }

    fn xread(&self, addr: Word, mode: MemMode) -> Result<Word, MemFault> {
        self.mem.get(addr as usize)
                .ok_or(MemFault{ addr, mode })
                .map(|ptr| *ptr)
    }

    fn iread(&self, addr: Word) -> Result<Word, MemFault> {
        self.xread(addr, MemMode::IRead)
    }

    pub fn read(&self, addr: Word) -> Result<Word, MemFault> {
        self.xread(addr, MemMode::DRead)
    }

    pub fn write(&mut self, addr: Word, val: Word) -> Result<(), MemFault> {
        self.mem.get_mut(addr as usize)
                .ok_or(MemFault{ addr, mode: MemMode::DWrite })
                .map(|ptr| *ptr = val)
    }

    fn read_param(&self, insn: &Insn, idx: usize) -> Result<Word, ExecFault> {
        let field = self.iread(self.pc + 1 + (idx as Word))?;
        match insn.modes[idx] {
            Mode::Immediate => Ok(field),
            Mode::Position => Ok(self.read(field)?),
        }
    }

    fn write_param(&mut self, insn: &Insn, idx: usize, val: Word) -> Result<(), ExecFault> {
        let field = self.iread(self.pc + (idx as Word))?;
        match insn.modes[idx] {
            Mode::Immediate => Err(ExecFault::WriteImmediate),
            Mode::Position => Ok(self.write(field, val)?),
        }
    }

    fn exec(&mut self, io: &mut dyn Device) -> Result<Stepped, ExecFault> {
        let insn = Insn::decode(self.iread(self.pc)?)?;
        match insn.opcode {
            Opcode::Add =>
                self.write_param(&insn, 2,
                                 self.read_param(&insn, 0)? +
                                 self.read_param(&insn, 1)?),
            Opcode::Mul => 
                self.write_param(&insn, 2,
                                 self.read_param(&insn, 0)? *
                                 self.read_param(&insn, 1)?),
            Opcode::In =>
                self.write_param(&insn, 0, io.input()?),
            Opcode::Out =>
                Ok(io.output(self.read_param(&insn, 0)?)?),
            Opcode::Halt =>
                return Ok(Stepped::Halted),
        }?;
        self.pc += insn.opcode.len();
        return Ok(Stepped::Ok);
    }

    pub fn step(&mut self, io: &mut dyn Device) -> Result<Stepped, ExecError> {
        let pc = self.pc;
        self.exec(io).map_err(|fault| ExecError { pc, fault })
    }

    pub fn run(&mut self, io: &mut dyn Device) -> Result<(), ExecError> {
        loop {
            match self.step(io)? {
                Stepped::Ok => (),
                Stepped::Halted => return Ok(()),
            }
        }
    }

}

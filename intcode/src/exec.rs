use std::convert::From;

use crate::{Word, ParseError, decode::{Insn, Mode, Opcode, DecodeFault}};

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
    Overflow(ArithOp, Word, Word),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithOp {
    Add,
    Mul,
    Lea,
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

#[derive(Clone)]
pub struct Computer {
    pc: Word,
    base: Word,
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

fn setcc(b: bool) -> Word {
    if b { 1 } else { 0 }
}

fn alu_add(x: Word, y: Word) -> Result<Word, ExecFault> {
    x.checked_add(y).ok_or(ExecFault::Overflow(ArithOp::Add, x, y))
}

fn alu_mul(x: Word, y: Word) -> Result<Word, ExecFault> {
    x.checked_mul(y).ok_or(ExecFault::Overflow(ArithOp::Mul, x, y))
}

impl Computer {
    pub fn new(mem: Vec<Word>) -> Self {
        assert!(mem.len() - 1 <= Word::max_value() as usize);
        Self { pc: 0, base: 0, mem }
    }

    pub fn from_str(s: &str) -> Result<Self, ParseError> {
        Ok(Self::new(crate::parse(s)?))
    }

    fn xread(&self, addr: Word, mode: MemMode) -> Result<Word, MemFault> {
        if addr < 0 {
            return Err(MemFault{ addr, mode });
        }
        Ok(self.mem.get(addr as usize)
           .map(|ptr| *ptr)
           .unwrap_or(0))
    }

    fn iread(&self, pcrel: Word) -> Result<Word, MemFault> {
        self.xread(self.pc + pcrel, MemMode::IRead)
    }

    pub fn read(&self, addr: Word) -> Result<Word, MemFault> {
        self.xread(addr, MemMode::DRead)
    }

    pub fn write(&mut self, addr: Word, val: Word) -> Result<(), MemFault> {
        if addr < 0 {
            return Err(MemFault{ addr, mode: MemMode::DWrite });
        }
        let uaddr = addr as usize;
        if uaddr >= self.mem.len() {
            self.mem.resize(uaddr + 1, 0);
        }
        Ok(self.mem[uaddr] = val)
    }

    fn lea_rel(&self, field: Word) -> Result<Word, ExecFault> {
        self.base.checked_add(field).ok_or(ExecFault::Overflow(ArithOp::Lea, self.base, field))
    }

    fn read_param(&self, insn: &Insn, idx: usize) -> Result<Word, ExecFault> {
        let field = self.iread(1 + idx as Word)?;
        match insn.modes[idx] {
            Mode::Immediate => Ok(field),
            Mode::Position => Ok(self.read(field)?),
            Mode::Relative => Ok(self.read(self.lea_rel(field)?)?),
        }
    }

    fn write_param(&mut self, insn: &Insn, idx: usize, val: Word) -> Result<(), ExecFault> {
        let field = self.iread(1 + idx as Word)?;
        match insn.modes[idx] {
            Mode::Immediate => Err(ExecFault::WriteImmediate),
            Mode::Position => Ok(self.write(field, val)?),
            Mode::Relative => Ok(self.write(self.lea_rel(field)?, val)?),
        }
    }

    fn exec(&mut self, io: &mut dyn Device) -> Result<Stepped, ExecFault> {
        let insn = Insn::decode(self.iread(0)?)?;
        let mut npc = self.pc + insn.opcode.len();
        match insn.opcode {
            Opcode::Add =>
                self.write_param(&insn, 2,
                                 alu_add(self.read_param(&insn, 0)?,
                                         self.read_param(&insn, 1)?)?),
            Opcode::Mul => 
                self.write_param(&insn, 2,
                                 alu_mul(self.read_param(&insn, 0)?,
                                         self.read_param(&insn, 1)?)?),
            Opcode::In =>
                self.write_param(&insn, 0, io.input()?),
            Opcode::Out =>
                Ok(io.output(self.read_param(&insn, 0)?)?),
            Opcode::Jnz =>
                Ok(if self.read_param(&insn, 0)? != 0 {
                    npc = self.read_param(&insn, 1)?;
                }),
            Opcode::Jz =>
                Ok(if self.read_param(&insn, 0)? == 0 {
                    npc = self.read_param(&insn, 1)?;
                }),
            Opcode::CmpLt =>
                self.write_param(&insn, 2,
                                 setcc(self.read_param(&insn, 0)? <
                                       self.read_param(&insn, 1)?)),
            Opcode::CmpEq =>
                self.write_param(&insn, 2,
                                 setcc(self.read_param(&insn, 0)? ==
                                       self.read_param(&insn, 1)?)),
            Opcode::SetBase =>
                Ok(self.base += self.read_param(&insn, 0)?),
            Opcode::Halt =>
                return Ok(Stepped::Halted),
        }?;
        self.pc = npc;
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

    pub fn into_mem(self) -> Vec<Word> {
        self.mem
    }
}

// TODO, maybe: tests for the error cases

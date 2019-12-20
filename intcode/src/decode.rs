use crate::Word;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Insn {
    pub opcode: Opcode,
    pub modes: [Mode; 3],
} 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Position,
    Immediate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Add,
    Mul,
    In,
    Out,
    Halt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeFault {
    Unknown(Word),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpcodeFault {
    Unknown(Word),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeFault {
    Negative(Word),
    Mode{ param: usize, fault: ModeFault },
    Opcode(OpcodeFault),
    ReservedNonZero(Word),
}

impl Mode {
    pub fn decode(w: Word) -> Result<Self, ModeFault> {
        match w {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            _ => Err(ModeFault::Unknown(w))
        }
    }
}

impl Opcode {
    pub fn decode(w: Word) -> Result<Self, OpcodeFault> {
        match w {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Mul),
            3 => Ok(Opcode::In),
            4 => Ok(Opcode::Out),
            99 => Ok(Opcode::Halt),
            _ => Err(OpcodeFault::Unknown(w))
        }
    }
}

impl Insn {
    pub fn decode(w: Word) -> Result<Self, DecodeFault> {
        if w < 0 {
            return Err(DecodeFault::Negative(w));
        }
        let mut a = w;
        let opcode = Opcode::decode(a % 100).map_err(DecodeFault::Opcode)?;
        a /= 100;
        let mut modes = [None; 3];
        for i in 0..3 {
            let eee = |e| DecodeFault::Mode{ param: i, fault: e };
            modes[i] = Some(Mode::decode(a % 10).map_err(eee)?);
            a /= 10;
        }
        if a != 0 {
            return Err(DecodeFault::ReservedNonZero(a));
        }
        Ok(Self {
            opcode,
            modes: [modes[0].unwrap(),
                    modes[1].unwrap(),
                    modes[2].unwrap()]
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_errors() {
        assert_eq!(Insn::decode(0),
                   Err(DecodeFault::Opcode(OpcodeFault::Unknown(0))));
        assert_eq!(Insn::decode(21),
                   Err(DecodeFault::Opcode(OpcodeFault::Unknown(21))));
        assert_eq!(Insn::decode(201),
                   Err(DecodeFault::Mode{ param: 0, fault: ModeFault::Unknown(2)}));
        assert_eq!(Insn::decode(2001),
                   Err(DecodeFault::Mode{ param: 1, fault: ModeFault::Unknown(2)}));
        assert_eq!(Insn::decode(20001),
                   Err(DecodeFault::Mode{ param: 2, fault: ModeFault::Unknown(2)}));
        assert_eq!(Insn::decode(200001),
                   Err(DecodeFault::ReservedNonZero(2)));
        assert_eq!(Insn::decode(-1),
                   Err(DecodeFault::Negative(-1)));
        assert_eq!(Insn::decode(Word::min_value()),
                   Err(DecodeFault::Negative(Word::min_value())));
    }

    #[test]
    fn test_successes() {
        assert_eq!(Insn::decode(2),
                   Ok(Insn{ opcode: Opcode::Mul,
                            modes: [Mode::Position,
                                    Mode::Position,
                                    Mode::Position]
                   }));
        assert_eq!(Insn::decode(1002),
                   Ok(Insn{ opcode: Opcode::Mul,
                            modes: [Mode::Position,
                                    Mode::Immediate,
                                    Mode::Position]
                   })); 
        assert_eq!(Insn::decode(10002),
                   Ok(Insn{ opcode: Opcode::Mul,
                            modes: [Mode::Position,
                                    Mode::Position,
                                    Mode::Immediate] // faults on execution
                   })); 
    }
}

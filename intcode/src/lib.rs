use std::num::ParseIntError;
use std::str::FromStr;

pub mod decode;
pub mod exec;

pub use exec::{Computer, Device, ExecError, IOError, Stepped};

pub type Word = i64;

pub type ParseError = ParseIntError;

pub fn parse(s: &str) -> Result<Vec<Word>, ParseError> {
    let mut acc = vec![];
    for num_or_err in s.split(",").map(|s| Word::from_str(s.trim())) {
        acc.push(num_or_err?);
    }
    Ok(acc)
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestDev {
        orig_in: Vec<Word>,
        in_tape: Vec<Word>,
        out_tape: Vec<Word>,
    }
    impl TestDev {
        fn new(orig_in: Vec<Word>) -> Self {
            let mut in_tape = orig_in.clone();
            in_tape.reverse();
            Self { orig_in, in_tape, out_tape: vec![] }
        }
        fn expect(&self, outputs: Vec<Word>) {
            assert_eq!(self.in_tape, vec![], "input was {:?}", self.orig_in);
            assert_eq!(self.out_tape, outputs, "input was {:?}", self.orig_in);
        }
    }
    impl Device for TestDev {
        fn input(&mut self) -> Result<Word, IOError> {
            self.in_tape.pop().ok_or(IOError)
        }
        fn output(&mut self, val: Word) -> Result<(), IOError> {
            Ok(self.out_tape.push(val))
        }
    }

    #[test]
    fn echo() {
        let mut dev = TestDev::new(vec![0xDEADBEE]);
        let mut cpu = Computer::new(vec![3,0,4,0,99]);
        cpu.run(&mut dev).unwrap();
        dev.expect(vec![0xDEADBEE]);
    }

    #[test]
    fn mul_imm() {
        let mut cpu = Computer::new(vec![1002,4,3,4,33]);
        cpu.run(&mut ()).unwrap();
        assert_eq!(cpu.read(4).unwrap(), 99);
    }

    #[test]
    fn add_neg() {
        let mut cpu = Computer::new(vec![1101,100,-1,4,0]);
        cpu.run(&mut ()).unwrap();
        assert_eq!(cpu.read(4).unwrap(), 99);
    }

    fn day2_case(before: Vec<Word>, after: Vec<Word>) {
        let mut cpu = Computer::new(before);
        cpu.run(&mut ()).unwrap();
        assert_eq!(cpu.into_mem(), after);
    }

    #[test]
    fn day2_line1() {
        day2_case(vec![1,0,0,0,99], vec![2,0,0,0,99]);
    }

    #[test]
    fn day2_line2() {
        day2_case(vec![2,3,0,3,99], vec![2,3,0,6,99]);
    }

    #[test]
    fn day2_line3() {
        day2_case(vec![2,4,4,5,99,0], vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn day2_line4() {
        day2_case(vec![1,1,1,4,99,5,6,0,99], vec![30,1,1,4,2,5,6,0,99]);
    }

    fn unary_check(prog: &[Word], inputs: &[Word], model: &dyn Fn(Word) -> Word) {
        for &i in inputs {
            let mut dev = TestDev::new(vec![i]);
            let mut cpu = Computer::new(prog.to_owned());
            cpu.run(&mut dev).unwrap();
            dev.expect(vec![model(i)]);
        }
    }

    #[test]
    fn d5p2_eq8_pos() {
        unary_check(&[3,9,8,9,10,9,4,9,99,-1,8],
                    &[8, 7, 9, 0, -8], &|i| if i == 8 { 1 } else { 0 });
    }

    #[test]
    fn d5p2_lt8_pos() {
        unary_check(&[3,9,7,9,10,9,4,9,99,-1,8],
                    &[8, 7, 9, 0, -8], &|i| if i < 8 { 1 } else { 0 });
    }

    #[test]
    fn d5p2_eq8_imm() {
        unary_check(&[3,3,1108,-1,8,3,4,3,99],
                    &[8, 7, 9, 0, -8], &|i| if i == 8 { 1 } else { 0 });
    }

    #[test]
    fn d5p2_lt8_imm() {
        unary_check(&[3,3,1107,-1,8,3,4,3,99],
                    &[8, 7, 9, 0, -8], &|i| if i < 8 { 1 } else { 0 });
    }

    #[test]
    fn d5p2_jmp_pos() {
        unary_check(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9],
                    &[0, 1, -1], &|i| if i == 0 { 0 } else { 1 })
    }
 
    #[test]
    fn d5p2_jmp_imm() {
        unary_check(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1],
                    &[0, 1, -1], &|i| if i == 0 { 0 } else { 1 })
    }

    #[test]
    fn d5p2_larger() {
        unary_check(&[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                      1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                      999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99],
                    &[8, 7, 9, 0, -8], &|i| {
                        if i < 8 { 999 }
                        else if i == 8 { 1000 }
                        else { 1001 }
                    });
    }

    #[test]
    fn d9_selfrep() {
        let prog = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut cpu = Computer::new(prog.clone());
        let mut dev = TestDev::new(vec![]);
        cpu.run(&mut dev).unwrap();
        dev.expect(prog);
    }

    #[test]
    fn d9_bigmul() {
        struct Dev16;
        impl Device for Dev16 {
            fn input(&mut self) -> Result<Word, IOError> { Err(IOError) }
            fn output(&mut self, val: Word) -> Result<(), IOError> {
                let pval = format!("{}", val);
                Ok(assert_eq!(pval.len(), 16, "bad value: {}", val))
            }
        }

        let mut cpu = Computer::new(vec![1102,34915192,34915192,7,4,7,99,0]);
        cpu.run(&mut Dev16).unwrap();
    }

    #[test]
    fn d9_biglit() {
        let mut cpu = Computer::from_str("104,1125899906842624,99").unwrap();
        let mut dev = TestDev::new(vec![]);
        cpu.run(&mut dev).unwrap();
        dev.expect(vec![1125899906842624]);
    }
}

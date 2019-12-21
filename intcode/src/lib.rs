pub mod decode;
pub mod exec;

pub use exec::{Computer, Device, ExecError, IOError, Stepped};

pub type Word = i32;

#[cfg(test)]
mod test {
    use super::*;

    struct TestDev {
        in_tape: Vec<Word>,
        out_tape: Vec<Word>,
    }
    impl TestDev {
        fn new(inputs: Vec<Word>) -> Self {
            let mut in_tape = inputs.to_owned();
            in_tape.reverse();
            Self { in_tape, out_tape: vec![] }
        }
        fn expect(&self, outputs: Vec<Word>) {
            assert_eq!(self.in_tape, vec![]);
            assert_eq!(self.out_tape, outputs);
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

}

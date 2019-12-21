use std::mem::drop;
use std::{thread, sync::mpsc};

use intcode::{Computer, Device, IOError, Word};

struct ParDev {
    recv: mpsc::Receiver<Word>,
    send: mpsc::Sender<Word>,
    phase: Option<Word>,
    last: Option<Option<Word>>,
}

impl Device for ParDev {
    fn input(&mut self) -> Result<Word, IOError> {
        if let Some(phase) = self.phase.take() {
            return Ok(phase + 5);
        }
        Ok(self.recv.recv().expect("recv error"))
    }
    fn output(&mut self, val: Word) -> Result<(), IOError> {
        let send_result = self.send.send(val);
        Ok(if let Some(ref mut last) = self.last {
            *last = Some(val);
        } else {
            send_result.expect("send error");
        })
    }
}

pub fn par_amp(cpu: &Computer, phases: &[Word]) -> Word {
    let mut sends = vec![];
    let mut recvs = vec![];

    let n = phases.len();
    for _i in 0..n {
        let (s, r) = mpsc::channel::<Word>();
        sends.push(Some(s));
        recvs.push(Some(r));
    }
    let mut joins = vec![];
    let (final_s, final_r) = mpsc::channel::<Word>();
    let init_s = sends[0].clone().unwrap();
    for i in 0..n {
        let recv = recvs[i].take().unwrap();
        let send = sends[(i + 1) % n].take().unwrap();
        let cpu = cpu.clone();
        let final_s = if i == n - 1 { Some(final_s.clone()) } else { None };
        let phase = Some(phases[i]);
        joins.push(thread::spawn(move || {
            let mut cpu = cpu;
            let mut dev = ParDev {
                recv, send, phase, last: final_s.as_ref().map(|_| None)
            };
            cpu.run(&mut dev).expect("magic smoke escaped");
            if let Some(final_s) = final_s {
                final_s.send(dev.last.unwrap().expect("no output")).expect("final send error");
            }
        }));
    }
    drop(final_s);
    init_s.send(0).expect("initial send error");
    drop(init_s);
    let last = final_r.recv().expect("final recv error");
    for join in joins {
        join.join().expect("join error");
    }
    return last;
}

#[cfg(test)]
mod test {
    use super::par_amp;
    use crate::permutations;
    use intcode::{Computer, Word};

    fn thrustify(prog: Vec<Word>, best: Word, when: &[Word]) {
        let cpu = Computer::new(prog);
        assert_eq!(par_amp(&cpu, when), best);
        for other in permutations(when.len() as Word) {
            assert!(par_amp(&cpu, &other) <= best, "input {:?} exceeded {:?}", other, best)
        }
    }

    #[test]
    fn example4() {
        thrustify(vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                       27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5],
                  139629729, &[4,3,2,1,0]);
    }


    #[test]
    fn example5() {
        thrustify(vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                       -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                       53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10],
                  18216, &[4,2,3,0,1]);
    }
}

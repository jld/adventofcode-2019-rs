use std::mem::drop;
use std::{thread, sync::{mpsc, Arc}};

use intcode::{Computer, Device, IOError, Word};

struct ParDev {
    recv: mpsc::Receiver<Word>,
    send: mpsc::Sender<Word>,
    phase: Option<Word>,
    last: Option<Word>,
}

impl Device for ParDev {
    fn input(&mut self) -> Result<Word, IOError> {
        if let Some(phase) = self.phase.take() {
            return Ok(phase + 5);
        }
        Ok(self.recv.recv().expect("recv error"))
    }
    fn output(&mut self, val: Word) -> Result<(), IOError> {
        self.last = Some(val);
        Ok(self.send.send(val).expect("send error"))
    }
}

fn par_amp(cpu: Computer, phases: &[Word]) -> Word {
    let mut sends = vec![];
    let mut recvs = vec![];

    let n = phases.len();
    for _i in 0..n {
        let (s, r) = mpsc::channel::<Word>();
        sends.push(Some(s));
        recvs.push(Some(r));
    }
    let cpu = Arc::new(cpu);
    let mut joins = vec![];
    let (final_s, final_r) = mpsc::channel::<Word>();
    for i in 0..n {
        let recv = recvs[i].take().unwrap();
        let send = sends[(i + 1) % n].take().unwrap();
        let cpu = cpu.clone();
        let final_s = final_s.clone();
        let phase = Some(phases[i]);
        joins.push(thread::spawn(move || {
            let mut cpu = Computer::clone(&cpu);
            let mut dev = ParDev {
                recv, send, phase, last: None
            };
            cpu.run(&mut dev).expect("magic smoke escaped");
            if i == (n - 1) {
                final_s.send(dev.last.expect("no output")).expect("final send error");
            }
        }));
    }
    drop(final_s);
    let last = final_r.recv().expect("final recv error");
    for join in joins {
        join.join().expect("join error");
    }
    return last;
}

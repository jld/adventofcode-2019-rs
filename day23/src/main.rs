use std::collections::VecDeque;
use std::cell::RefCell;
use std::io::{stdin, prelude::*};

use intcode::{Computer, Device, Word, IOError, exec::Stepped};

struct NetQueue {
    queue: VecDeque<Word>,
    idling: bool,
}

impl NetQueue {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            idling: false
        }
    }

    fn pop(&mut self) -> Word {
        let maybe_word = self.queue.pop_front();
        self.idling = maybe_word.is_none();
        maybe_word.unwrap_or(-1)
    }

    fn push(&mut self, w: Word) {
        self.idling = false;
        self.queue.push_back(w);
    }

    fn is_idle(&self) -> bool {
        self.idling
    }
}

type Crossbar<'q> = &'q[RefCell<NetQueue>];

struct NetDev<'q> {
    crossbar: Crossbar<'q>,
    cmd_buf: Vec<Word>,
    addr: usize,
}

fn xlate_addr(w: Word) -> usize {
    if w == 255 {
        0
    } else {
        assert!(w >= 0 && w < 255);
        (w as usize) + 1
    }
}

impl<'q> NetDev<'q> {
    fn new(crossbar: Crossbar<'q>, addr: Word) -> Self {
        Self {
            crossbar,
            cmd_buf: vec![],
            addr: xlate_addr(addr)
        }
    }
}

impl<'q> Device for NetDev<'q> {
    fn output(&mut self, val: Word) -> Result<(), IOError> {
        debug_assert!(self.cmd_buf.len() < 3);
        self.cmd_buf.push(val);
        if self.cmd_buf.len() == 3 {
            let dest = xlate_addr(self.cmd_buf[0]);
            let mut queue = self.crossbar.get(dest).ok_or(IOError)?.borrow_mut();
            let x = self.cmd_buf[1];
            let y = self.cmd_buf[2];
            self.cmd_buf.clear();
            // eprintln!("{} -> {}: {} {}", self.addr, dest, x, y);
            queue.push(x);
            queue.push(y);
        }
        Ok(())
    }

    fn input(&mut self) -> Result<Word, IOError> {
        Ok(self.crossbar[self.addr].borrow_mut().pop())
    }
}

struct Nat<'q> {
    crossbar: Crossbar<'q>,
    size: Word,
    last: Option<(Word, Word)>
}

impl<'q> Nat<'q> {
    fn new(crossbar: Crossbar<'q>, size: Word) -> Self {
        Self {
            crossbar,
            size,
            last: None,
        }
    }

    fn tick(&mut self) {
        let mut my_queue = self.crossbar[0].borrow_mut();
        loop {
            let x = my_queue.pop();
            if x < 0 {
                break;
            }
            let y = my_queue.pop();
            println!("NAT recv: {} {}", x, y);
            self.last = Some((x, y));
        }
        if (0..self.size).all(|i| self.crossbar[xlate_addr(i)].borrow().is_idle()) {
            let mut q0 = self.crossbar[xlate_addr(0)].borrow_mut();
            if let &Some((x, y)) = &self.last {
                println!("NAT send: {} {}", x, y);
                q0.push(x);
                q0.push(y);
            }
        }
    }
}

fn main() {
    let stdin = stdin();
    let prog = stdin.lock().lines().next().expect("no input").expect("I/O error reading stdin");
    let cpu = Computer::from_str(&prog).expect("parse error");

    let crossbar: Vec<_> = (0..51).map(|_| RefCell::new(NetQueue::new())).collect();
    let mut cpus: Vec<_> = (0..50).map(|_| cpu.clone()).collect();
    let mut devs: Vec<_> = (0..50).map(|a| NetDev::new(&crossbar, a)).collect();
    let mut nat = Nat::new(&crossbar, 50);

    for i in 0..50 {
        crossbar[xlate_addr(i)].borrow_mut().push(i);
    }

    loop {
        for i in 0..50 {
            match cpus[i].step(&mut devs[i]) {
                Ok(Stepped::Ok) => (),
                Ok(Stepped::Halted) => panic!("CPU{} halted", i),
                e @ Err(_) => { e.expect(&format!("CPU{} fault", i)); }
            }
        }
        nat.tick();
    }
}

use std::collections::HashMap;
use std::env::args;
use std::fs::OpenOptions;
use std::io::{stdin, prelude::*, BufReader, BufWriter};
use std::ops::Drop;

use intcode::{Computer, Device, Word, IOError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HPad,
    Ball,
}

impl Tile {
    fn from_word(w: Word) -> Option<Self> {
        Some (match w {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HPad,
            4 => Tile::Ball,
            _ => return None
        })
    }
}

#[derive(Debug, Clone)]
struct CmdBuf {
    buf: Vec<Word>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cmd {
    None,
    Draw(Word, Word, Tile),
    Status(Word)
}

impl CmdBuf {
    fn new() -> Self { Self { buf: vec![] } }

    fn handle(&mut self, val: Word) -> Result<Cmd, IOError> {
        debug_assert!(self.buf.len() < 3);
        self.buf.push(val);
        Ok(if self.buf.len() == 3 {
            let x = self.buf[0];
            let y = self.buf[1];
            let data = self.buf[2];
            self.buf.clear();

            if x == -1 && y == 0 {
                Cmd::Status(data)
            } else if x < 0 || y < 0 {
                return Err(IOError);
            } else {
                let tile = Tile::from_word(data).ok_or(IOError)?;
                Cmd::Draw(x, y, tile)
            }
        } else {
            Cmd::None
        })
    }
}

#[derive(Debug, Clone)]
struct ScreenDev {
    tiles: HashMap<(Word, Word), Tile>,
    cmd: CmdBuf
}

impl ScreenDev {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            cmd: CmdBuf::new(),
        }
    }
}

impl Device for ScreenDev {
    fn input(&mut self) -> Result<Word, IOError> { Err(IOError) }

    fn output(&mut self, val: Word) -> Result<(), IOError> {
        Ok(match self.cmd.handle(val)? {
            Cmd::Draw(x, y, tile) => { self.tiles.insert((x, y), tile); }
            Cmd::None => (),
            _ => return Err(IOError),
        })
    }
}

struct TermDev {
    cmd: CmdBuf,
    line_in: Box<dyn BufRead>,
    vt_out: Box<dyn Write>,
    cursor: Option<(Word, Word)>,
    status: Word,
    lowest: Word,
}

type TermAction = std::io::Result<()>;

impl TermDev {
    fn new(r: impl BufRead + 'static, w: impl Write + 'static) -> Self {
        Self {
            cmd: CmdBuf::new(),
            line_in: Box::new(r),
            vt_out: Box::new(w),
            cursor: None,
            status: -1,
            lowest: -1,
        }
    }

    fn cls(&mut self) -> TermAction {
        write!(&mut self.vt_out, "\x1b[2J")
    }

    fn goto(&mut self, x: Word, y: Word) -> TermAction {
        debug_assert!(x >= 0 && y >= 0);
        if let Some((cx, cy)) = self.cursor {
            if cx == x && cy == y {
                return Ok(())
            }
        } else {
            self.cls()?;
        }
        write!(&mut self.vt_out, "\x1b[{};{}H", y+1, x+1)?;
        self.cursor = Some((x, y));
        Ok(())
    }

    fn put_text(&mut self, x: Word, y: Word, text: &str, width: Word) -> TermAction {
        self.goto(x, y)?;
        write!(&mut self.vt_out, "{}", text)?;
        self.cursor.as_mut().unwrap().0 += width;
        Ok(())
    }
    
    fn write_status(&mut self, prompt: &str) -> TermAction {
        self.goto(0, self.lowest + 1)?;
        write!(&mut self.vt_out, "{}{}\x1b[K", self.status, prompt)?;
        self.vt_out.flush()?;
        self.cursor.as_mut().unwrap().0 = -1;
        Ok(())
    }
}

impl Device for TermDev {
    fn output(&mut self, val: Word) -> Result<(), IOError> {
        Ok(match self.cmd.handle(val)? {
            Cmd::None => (),
            Cmd::Status(status) => self.status = status,
            Cmd::Draw(x, y, tile) => {
                self.lowest = self.lowest.max(y);
                let text = match tile {
                    Tile::Empty => ".",
                    Tile::Wall => "#",
                    Tile::Block => "$",
                    Tile::HPad => "=",
                    Tile::Ball => "*",
                };
                self.put_text(x, y, text, 1).expect("tty write error");
            }
        })
    }

    fn input(&mut self) -> Result<Word, IOError> {
        self.write_status("> ").expect("tty write error");
        // I'm too lazy to put the tty into cbreak mode, so line input it is.
        let mut line = String::new();
        self.line_in.read_line(&mut line).expect("tty read error");
        let joy = match line.chars().next().unwrap().to_ascii_lowercase() {
            'l' | 'w' => -1,
            'r' | 'e' => 1,
            'n' | 'h' => 0,
            _ => panic!("unknown direction {:?}", line.trim())
        };
        Ok(joy)
    }
}

impl Drop for TermDev {
    fn drop(&mut self) {
        // Ignore all error here because double panic isn't useful.
        let _ = self.write_status("");
        // Move the cursor back down in case there's a panic message under the status line
        let _ = write!(&mut self.vt_out, "\n\n");
        let _ = self.vt_out.flush();
    }
}

fn main() {
    // I really should clean up all this code duplication....
    let stdin = stdin();
    let prog = stdin.lock().lines().next().expect("no input").expect("I/O error reading stdin");
    let mut cpu = Computer::from_str(&prog).expect("parse error");
    std::mem::drop(stdin);

    let cmd = args().nth(1).expect("need argument: blocks | play");
    if cmd == "blocks" {
        let mut dev = ScreenDev::new();
        cpu.run(&mut dev).expect("runtime error");
        println!("{}", dev.tiles.iter().filter(|&(_xy, &t)| t == Tile::Block).count());
    } else if cmd == "play" {
        let tty_in = OpenOptions::new().read(true).open("/dev/tty")
                                                  .expect("error opening /dev/tty for read");
        let tty_out = OpenOptions::new().write(true).open("/dev/tty")
                                                    .expect("error opening /dev/tty for write");
        let mut dev = TermDev::new(BufReader::new(tty_in), BufWriter::new(tty_out));
        cpu.write(0, 2).unwrap();
        cpu.run(&mut dev).expect("runtime error");
    } else {
        panic!("bad command {}", cmd);
    }
}

use std::io;
use std::env::args;
use std::io::Read;

fn read_from_stdin(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

fn read_line() -> Result<String, ()> {
    let mut n = String::new();
    read_from_stdin(&mut n).map_err(|e| ())?;
    if n.len() == 0 {
        return Err(());
    }
    Ok(n.trim().to_string())
}

fn read_line_by<F>(mut f: F) -> Result<(), ()>
    where F: FnMut(u32, u8) {
    let mut byte = [0u8];
    let mut i = 0;
    loop {
        let len = io::stdin().read(&mut byte).map_err(|e| ())?;
        if len==0 || byte[0] == '\n' as u8 {
            break;
        }
        //println!("{}", byte[0] as char);
        f(i, byte[0]);
        i = i + 1;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 2 && args[1] == "test".to_string() {
        test();
        return;
    }

    process();
}

#[derive(Debug, PartialEq)]
struct RotateBuffer {
    i: usize,
    len: usize,
    buff: Vec<u8>,
}

impl RotateBuffer {
    fn new(size: usize) -> Self {
        Self {
            i: 0,
            len: 0,
            buff: vec![0u8; size],
        }
    }

    fn push(&mut self, byte: u8) {
        if self.buff.len() == 0 {
            return;
        }
        self.buff[self.i] = byte;
        self.len = if self.len + 1 > self.buff.len() { self.buff.len() } else { self.len + 1 };
        self.i = (self.i + 1) % self.buff.len();
    }

    fn equals(&self, other: &[u8]) -> bool {
        let start = self.i + self.buff.len() - self.len;
        let mut result = String::new();
        for i in 0..self.len {
            let b = self.buff[(start + i) % self.buff.len()];
            if b != other[i] {
                return false;
            }
        }
        true
    }
}

fn process() -> Result<(), ()> {
    loop {
        let needle_len = read_line()?;
        //println!("-{}", needle_len);
        let needle = read_line()?;
        //println!("-{}", needle);

        let mut rb = RotateBuffer::new(needle.len());
        let mut found = false;
        let needle_bytes = needle.as_bytes();

        read_line_by(|i, b| {
            //println!("{} {}", i, b as char);
            rb.push(b);
            if rb.len == needle_bytes.len() {
                //cmp
                if rb.equals(needle_bytes) {
                    let index = i + 1 - needle.len() as u32;

                    found = true;
                    println!("{}", index);
                }
            }
        })?;

        if found {
            println!();
        }
    }
}


fn test() {
    println!("test");

    let mut r = RotateBuffer::new(4);
    r.push('a' as u8);
    r.push('b' as u8);
    r.push('c' as u8);
    r.push('d' as u8);
    r.push('e' as u8);
    assert_eq!(RotateBuffer { i: 1, len: 4, buff: vec![101, 98, 99, 100] }, r);

    assert_eq!(true, r.equals("bcde".as_bytes()));
}
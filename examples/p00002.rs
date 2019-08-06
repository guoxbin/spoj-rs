use std::io;

const SIZE: usize = 20000;

fn read_from_stdin(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

fn gen_primaries() -> [u8; SIZE] {
    let mut primaries: [u8; SIZE] = [0; SIZE];
    for j in 0..SIZE {
        let n = if j == 0 { 2 } else { j * 2 + 1 };

        let mut is_primary = true;
        for i in (0..SIZE).filter(|i| primaries[*i] != 0) {
            let i = if i == 0 { 2 } else { i * 2 + 1 };
            if i > n {
                break;
            }
            if i * i > n {
                break;
            }
            if n % i == 0 {
                is_primary = false;
                break;
            }
        }
        primaries[j] = is_primary as u8;
    }
    primaries
}

fn solve(start: u32, end: u32, primaries: &mut [u8; SIZE]) {
    for n in start..=end {
        let n = n as usize;
        let is_primary = match n{
            1 => false,
            _ => {
                let mut is_primary = true;
                for i in (0..SIZE).filter(|i| primaries[*i] != 0) {
                    let i = if i == 0 { 2 } else { i * 2 + 1 };
                    if i > n {
                        break;
                    }
                    if i * i > n {
                        break;
                    }
                    if n % i == 0 {
                        is_primary = false;
                        break;
                    }
                }
                is_primary
            }
        };
        if is_primary {
            println!("{}", n);
        }
    }
    println!("");
}

fn main() {
    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();
    let len: usize = buf.trim().parse().unwrap();

    let mut primaries = gen_primaries();

    for _i in 0..len {
        buf.clear();
        read_from_stdin(&mut buf).unwrap();
        let trimed = buf.trim();
        let i = trimed.find(" ").unwrap();
        let start: u32 = (&trimed[0..i]).parse().unwrap();
        let end: u32 = (&trimed[i + 1..]).parse().unwrap();
        solve(start, end, &mut primaries);
    }
}
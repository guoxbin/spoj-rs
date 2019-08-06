use std::io;

fn read_from_stdin(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

const A_BYTE: u8 = 'a' as u8;
const Z_BYTE: u8 = 'z' as u8;
const L_BYTE: u8 = '(' as u8;
const R_BYTE: u8 = ')' as u8;
const PLUS_BYTE: u8 = '+' as u8;
const MINUS_BYTE: u8 = '-' as u8;
const MULTIPLE_BYTE: u8 = '*' as u8;
const DIVIDE_BYTE: u8 = '/' as u8;
const POW_BYTE: u8 = '^' as u8;

#[derive(Debug)]
struct Unit {
    left_index: usize,
    x: Option<String>,
    y: Option<String>,
    op: Option<u8>,
}

fn parse(line: &str) -> String {
    let bytes = line.as_bytes();

    let mut stack: Vec<Unit> = Vec::new();

    for i in 0..bytes.len() {
        let c = bytes[i];
        //println!("{:?}", stack);
        match c {
            L_BYTE => {
                stack.push(Unit {
                    left_index: i,
                    x: None,
                    y: None,
                    op: None,
                })
            },
            A_BYTE...Z_BYTE => {
                let mut top_unit = stack.last_mut().unwrap();
                if let None = top_unit.op {
                    top_unit.x = Some(format!("{}", c as char));
                } else {
                    top_unit.y = Some(format!("{}", c as char));
                }
            },
            PLUS_BYTE | MINUS_BYTE | MULTIPLE_BYTE | DIVIDE_BYTE | POW_BYTE=> {
                let mut top_unit = stack.last_mut().unwrap();
                top_unit.op = Some(c);
            },
            R_BYTE => {
                let top_unit = stack.pop().unwrap();

                let x = format!("{}{}{}",  top_unit.x.unwrap(),  top_unit.y.unwrap(),  top_unit.op.unwrap() as char);

                match stack.last_mut(){
                    Some(top_unit) => {
                        if let None = top_unit.op {
                            top_unit.x = Some(x);
                        } else {
                            top_unit.y = Some(x);
                        }
                    },
                    None => return x,
                }
            },
            _ => unreachable!()
        }
    }
    "".to_string()
}

fn main() {
    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();
    let len: usize = buf.trim().parse().unwrap();

    for _i in 0..len {
        buf.clear();
        read_from_stdin(&mut buf).unwrap();
        let trimed = buf.trim();
        let parsed = parse(trimed);

        println!("{}", parsed);
    }
}

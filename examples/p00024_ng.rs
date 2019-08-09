use std::io;
use std::env::args;
use std::cmp::{max, min};

fn read_from_stdin(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

fn main() {
    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();
    let len: usize = buf.trim().parse().unwrap();

    let result = prepare(100);

    for _i in 0..len {
        buf.clear();
        read_from_stdin(&mut buf).unwrap();
        let trimed = buf.trim();
        let n: usize = trimed.parse().unwrap();

        let o = result[n - 1].output();
        println!("{}", o);
    }
}

fn byte_to_num(byte: u8) -> i8 {
    (byte - 48) as i8
}

fn num_to_byte(num: i8) -> u8 {
    (num + 48) as u8
}

#[derive(PartialEq, Clone, Debug)]
struct Num {
    s: Vec<i8>,
}

impl Num {
    fn parse(buf: &str) -> Self {
        let len = buf.len();
        let bytes = buf.as_bytes();
        let mut s = Vec::with_capacity(len);
        for i in 0..len {
            let j = len - 1 - i;
            s.push(byte_to_num(bytes[j]));
        }
        Num {
            s,
        }
    }

    fn output(&self) -> String {
        let len = self.s.len();

        let mut s = String::new();
        for i in 0..len {
            let j = len - 1 - i;
            s.push(num_to_byte(self.s[j]) as char);
        }
        s
    }
}

fn add(a: &Num, b: &Num) -> Num {
    let len = max(a.s.len(), b.s.len()) + 1;

    let mut result = vec![0i8; len];

    for i in 0..len {
        let a_digit = if i < a.s.len() { a.s[i] } else { 0i8 };
        let b_digit = if i < b.s.len() { b.s[i] } else { 0i8 };

        let sum = result[i] + a_digit + b_digit;
        let this = if sum >= 10 { sum - 10 } else { sum };
        let next = sum / 10;

        result[i] = this;
        if i < len - 1 {
            result[i + 1] = result[i + 1] + next;
        }
    }

    let mut prefix_zero_count = 0;
    for i in 0..len {
        let j = len - i - 1;
        if result[j] == 0 {
            prefix_zero_count = prefix_zero_count + 1;
        } else {
            break;
        }
    }
    if prefix_zero_count == len {
        prefix_zero_count = len - 1;
    }

    result.truncate(len - prefix_zero_count);

    Num {
        s: result,
    }
}

fn multiple_digit(a: &Num, b: i8) -> Num {
    if b == 0 {
        return Num {
            s: vec![0i8],
        };
    }

    let len = a.s.len() + 1;

    let mut result = Vec::new();
    for _i in 0..len {
        result.push(0i8);
    }

    for i in 0..len {
        let a_digit = if i < a.s.len() { a.s[i] } else { 0i8 };

        let mul = result[i] + a_digit * b;
        let this = if mul >= 10 { mul % 10 } else { mul };
        let next = mul / 10;

        result[i] = this;
        if i < len - 1 {
            result[i + 1] = result[i + 1] + next;
        }
    }

    let mut prefix_zero_count = 0;
    for i in 0..len {
        let j = len - i - 1;
        if result[j] == 0 {
            prefix_zero_count = prefix_zero_count + 1;
        } else {
            break;
        }
    }
    if prefix_zero_count == len {
        prefix_zero_count = len - 1;
    }

    result.truncate(len - prefix_zero_count);

    Num {
        s: result,
    }
}

fn multiple(a: &Num, b: &Num) -> Num {
    let mut sum = Num::parse("0");
    let mut i = 0;
    for bb in &b.s {
        let mul = multiple_digit(a, *bb);
        //add i zero
        let mut tmp = vec![0; i];
        tmp.extend(mul.s);
        sum = add(&sum, &Num { s: tmp });
        i = i + 1;
    }
    sum
}

fn prepare(max_n: usize) -> Vec<Num> {
    let mut result = vec![Num::parse("0"); max_n];

    let one = &Num::parse("1");

    let mut mul = Num::parse("1");
    let mut i_num = Num::parse("1");

    for i in 0usize..max_n {
        mul = multiple(&mul, &i_num);

        result[i] = mul.clone();

        i_num = add(&i_num, one);
    }
    result
}
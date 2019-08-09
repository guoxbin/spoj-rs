use std::io;
use std::env::args;
use std::cmp::max;

fn read_from_stdin(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

fn process(buf: &str) {

    let f = factorial(&Num::parse(buf));
    println!("{}", f);
}

fn byte_to_num(byte: u8) -> i8 {
    (byte - 48) as i8
}

fn num_to_byte(num: i8) -> u8 {
    (num + 48) as u8
}

#[derive(PartialEq)]
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
}

fn add(a: &Num, b: &Num) -> Num {
    let len = max(a.s.len(), b.s.len()) + 1;

    let mut result = Vec::new();
    for _i in 0..len {
        result.push(0i8);
    }

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
    assert_ne!(a.s[0], 0i8);

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

fn rm_end_zero(n: &Num) -> (Num, u32) {
    let mut s = Vec::new();
    let mut zero_count = 0;
    let mut stop = false;
    for i in &n.s {

        if *i==0 &&!stop{
            zero_count = zero_count + 1;
            continue;
        }
        stop = true;
        s.push(*i);
    }
    (Num {
        s,
    }, zero_count)
}

fn factorial(n: &Num) -> u32 {

    let mut mul = Num::parse("1");

    let mut x = Num::parse("1");

    let mut whole_zero_count = 0;
    loop{

        mul = multiple(&mul, &x);

        let r = rm_end_zero(&mul);

        mul = r.0;

        let zero_count = r.1;

        //println!("{:?} {}", mul.s, zero_count);

        whole_zero_count = whole_zero_count + zero_count;

        if &x == n{
            break;
        }
        x = add(&x, &Num::parse("1"));
    }

    whole_zero_count

}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 2 && args[1] == "test".to_string() {
        test();
        return;
    }

    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();
    let len: usize = buf.trim().parse().unwrap();

    for _i in 0..len {
        buf.clear();
        read_from_stdin(&mut buf).unwrap();
        let trimed = buf.trim();
        process(trimed);
    }
}

fn test() {
    println!("test");

    assert_eq!(vec![4, 3, 2, 1], Num::parse("1234").s);

    assert_eq!(vec![6, 3, 9, 4], multiple(&Num::parse("1234"), &Num::parse("4")).s);

    assert_eq!(vec![6, 5, 7, 2, 2, 5, 1], multiple(&Num::parse("1234"), &Num::parse("1234")).s);

    let r = rm_end_zero(&Num::parse("1234"));
    assert_eq!(vec![4, 3, 2, 1], r.0.s);
    assert_eq!(0, r.1);

    let r = rm_end_zero(&Num::parse("123400"));
    assert_eq!(vec![4, 3, 2, 1], r.0.s);
    assert_eq!(2, r.1);

    assert_eq!(0, factorial(&Num::parse("3")));

    assert_eq!(2, factorial(&Num::parse("10")));

    assert_eq!(14, factorial(&Num::parse("60")));
}
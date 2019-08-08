use std::io;
use std::cmp::{min, max};

fn read_from_stdin(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

fn process(buf: &str) {
    if buf.contains("+") {
        let a: Vec<&str> = buf.split("+").collect();
        output(
            output_add(
                Num::parse(a[0]),
                Num::parse(a[1]),
            )
        );
    } else if buf.contains("-") {
        let a: Vec<&str> = buf.split("-").collect();
        output(
            output_substract(
                Num::parse(a[0]),
                Num::parse(a[1]),
            )
        );
    } else if buf.contains("*") {
        let a: Vec<&str> = buf.split("*").collect();
        output(
            output_multiple(
                Num::parse(a[0]),
                Num::parse(a[1]),
            )
        );
    }
    println!();
}

fn output(o: Vec<String>) {
    for i in o {
        println!("{}", i);
    }
}

fn byte_to_num(byte: u8) -> i8 {
    (byte - 48) as i8
}

fn num_to_byte(num: i8) -> u8 {
    (num + 48) as u8
}

#[derive(Debug)]
struct Num {
    s: Vec<i8>
}

impl Num {
    fn parse(buf: &str) -> Self {
        let len = buf.len();
        let bytes = buf.as_bytes();
        let mut s = Vec::new();
        for i in 0..len {
            let j = len - i - 1;
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

fn substract(a: &Num, b: &Num) -> Num {
    let len = a.s.len();

    let mut result = a.s.clone();

    for i in 0..len {
        let b_digit = if i < b.s.len() { b.s[i] } else { 0i8 };

        let sub = result[i] - b_digit;
        let this = if sub < 0 { sub + 10 } else { sub };
        let next = if sub < 0 { 1 } else { 0 };

        result[i] = this;
        if i < len - 1 {
            result[i + 1] = result[i + 1] - next;
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

fn multiple(a: &Num, b: i8) -> Num {
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

fn output_add(a: Num, b: Num) -> Vec<String> {
    let result = add(&a, &b);

    let max_len = max(max(a.s.len(), b.s.len() + 1), result.s.len());

    let a_str = output_num(&a, max_len);

    let b_str = output_num_with_op(&b, '+', max_len);

    let seprator = output_seprator(max(b.s.len() + 1, result.s.len()), max_len);

    let result_str = output_num(&result, max_len);

    vec![
        a_str, b_str, seprator, result_str
    ]
}

fn output_substract(a: Num, b: Num) -> Vec<String> {
    let result = substract(&a, &b);

    let max_len = max(max(a.s.len(), b.s.len() + 1), result.s.len());

    let a_str = output_num(&a, max_len);

    let b_str = output_num_with_op(&b, '-', max_len);

    let seprator = output_seprator(max(b.s.len() + 1, result.s.len()), max_len);

    let result_str = output_num(&result, max_len);

    vec![
        a_str, b_str, seprator, result_str
    ]
}

fn output_multiple(a: Num, b: Num) -> Vec<String> {
    let b_len = b.s.len();

    let mut mul_result = Num::parse("0");

    let mut middle = Vec::new();

    for i in 0..b_len {
        let b_digit = b.s[i];

        let mul = multiple(&a, b_digit);

        middle.push((mul.s.clone(), i));

        //real mul
        let mut real_mul = Vec::new();
        for _j in 0..i {
            //insert j 0
            real_mul.push(0i8);
        }
        real_mul.extend(mul.s);

        let real_mul = Num {
            s: real_mul
        };

        mul_result = add(&mul_result, &real_mul);
    }

    let max_len = max(max(a.s.len(), b.s.len() + 1), mul_result.s.len());

    let mut result = Vec::new();

    result.push(output_num(&a, max_len));
    result.push(output_num_with_op(&b, '*', max_len));

    let middle_len = middle.len();

    for i in middle {
        if i.1 == 0 {
            result.push(output_seprator(max(b.s.len() + 1, i.0.len()), max_len));
        }

        let a = output_num(&Num { s: i.0 }, max_len);
        let a = a[i.1..].to_string();
        result.push(a);
    }

    if middle_len > 1 {
        result.push(output_seprator(mul_result.s.len(), max_len));
        result.push(output_num(&mul_result, max_len));
    }

    result
}

fn output_num(num: &Num, max_len: usize) -> String {
    let len = num.s.len();
    let mut r = "".to_string();
    for _i in 0..(max_len - len) {
        r.push(' ');
    }
    for i in 0..len {
        let j = len - i - 1;
        r.push(num_to_byte(num.s[j]) as char);
    }
    r
}

fn output_num_with_op(num: &Num, op: char, max_len: usize) -> String {
    let len = num.s.len() + 1;
    let mut r = "".to_string();
    for _i in 0..(max_len - len) {
        r.push(' ');
    }
    r.push(op);
    for i in 0..(len - 1) {
        let j = (len - 1) - i - 1;
        r.push(num_to_byte(num.s[j]) as char);
    }
    r
}

fn output_seprator(len: usize, max_len: usize) -> String {
    let mut r = "".to_string();
    for _i in 0..(max_len - len) {
        r.push(' ');
    }
    for _i in 0..len {
        r.push('-');
    }
    r
}

fn main() {
    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();
    let len: usize = buf.trim().parse().unwrap();

    for _i in 0..len {
        buf.clear();
        read_from_stdin(&mut buf).unwrap();
        let trimed = buf.trim();
        process(trimed);
    }

    //test();
}

fn test() {
    assert_eq!(vec![3i8, 2i8, 1i8], Num::parse("123").s);

    assert_eq!(vec![9, 7, 5], add(&Num::parse("123"), &Num::parse("456")).s);

    assert_eq!(vec![9, 7, 2, 1], add(&Num::parse("823"), &Num::parse("456")).s);

    assert_eq!(vec![7, 6, 3], substract(&Num::parse("823"), &Num::parse("456")).s);

    assert_eq!(vec![7, 6, 7], substract(&Num::parse("823"), &Num::parse("56")).s);

    let a = output_add(Num::parse("823"), Num::parse("456"));
    assert_eq!(vec![" 823", "+456", "----", "1279"], a);

    let a = output_add(Num::parse("823"), Num::parse("0"));
    assert_eq!(vec!["823", " +0", "---", "823"], a);

    let a = output_substract(Num::parse("823"), Num::parse("456"));
    assert_eq!(vec![" 823", "-456", "----", " 367"], a);

    let a = output_substract(Num::parse("823"), Num::parse("4"));
    assert_eq!(vec!["823", " -4", "---", "819"], a);

    assert_eq!(vec![5, 2, 6, 1], multiple(&Num::parse("325"), 5i8).s);

    let a = output_multiple(Num::parse("325"), Num::parse("4405"));

    assert_eq!(vec!["    325", "  *4405", "  -----", "   1625", "     0", " 1300", "1300", "-------", "1431625"], a);

    assert_eq!(vec![6, 3, 9, 4], multiple(&Num::parse("1234"), 4i8).s);

    assert_eq!(vec![0], add(&Num::parse("0"), &Num::parse("0")).s);

    assert_eq!(vec![0], substract(&Num::parse("0"), &Num::parse("0")).s);

    assert_eq!(vec![0], multiple(&Num::parse("0"), 1i8).s);
}
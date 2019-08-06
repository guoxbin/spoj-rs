use std::io;

const TEN: u8 = ('9' as u8 + 1);

fn read_from_stdin(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

fn process(line: &str) {
    if line == "" {
        println!();
        return;
    }
    let p = get_palindrome(line);
    println!("{}", p);
}

fn compare_low_high(b: &[u8]) -> i8 {
    let len = b.len();

    let mut result = 0;
    for i in (len - (len / 2))..len {
        if b[i] < b[len - i - 1] {
            result = -1;
            break;
        } else if b[i] == b[len - i - 1] {
            continue;
        } else {
            result = 1;
            break;
        }
    }

    result
}

fn make_high_big(b: &mut [u8]) {
    let len = b.len();
    let mut x = if len % 2 == 0 { len / 2 - 1 } else { len / 2 };
    loop {
        b[x] = b[x] + 1;
        if b[x] != TEN {
            break;
        }
        b[x] = '0' as u8;
        if x == 0 {
            break;
        }
        x = x - 1;
    }
    //println!("{:?}", b);
}

fn get_result(b: &[u8]) -> String {
    let len = b.len();

    if b[0] == '0' as u8 {
        let mut r = vec!['0' as u8; len + 1];
        r[0] = '1' as u8;
        r[len] = r[0];
        String::from_utf8(r).unwrap()
    } else {
        let mut r = vec![0u8; len];
        for i in (len - (len / 2))..len {
            r[i] = b[len - i - 1];
            r[len - i - 1] = r[i];
        }
        if len % 2 == 1 {
            r[len / 2] = b[len / 2];
        }
        String::from_utf8(r).unwrap()
    }
}

fn get_palindrome(line: &str) -> String {
    let b = &mut line.as_bytes().to_owned();

    let compare_result = compare_low_high(b);

    if compare_result < 0 {
        return get_result(b);
    } else {
        make_high_big(b);
        return get_result(b);
    }
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
}
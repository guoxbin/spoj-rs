use std::io;
use std::env::args;
use std::io::Read;

fn read_from_stdin(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
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
        let mut a = String::new();
        read_from_stdin(&mut a).unwrap();
        let a = a.trim();

        let mut b = String::new();
        read_from_stdin(&mut b).unwrap();
        let b = b.trim();

        process(&a, &b);
    }
}

fn process(a: &str, b: &str) {
    let mut r = find_longest_common(a.as_bytes(), 0, b.as_bytes(), 0);
    sort(&mut r);
    let r = uniq(r);
    for i in r {
        println!("{}", String::from_utf8(i).unwrap());
    }
    println!();
}

fn find_longest_common(a: &[u8], ai: usize, b: &[u8], bi: usize) -> Vec<Vec<u8>> {
    if ai >= a.len() || bi >= b.len() {
        return vec![vec![]];
    }

    if a[ai] == b[bi] {
        let r = find_longest_common(a, ai + 1, b, bi + 1);
        let mut result = Vec::new();
        for mut i in r {
            i.insert(0, a[ai]);
            result.push(i);
        }
        return result;
    }

    let mut r1 = find_longest_common(a, ai + 1, b, bi);
    let r2 = find_longest_common(a, ai, b, bi + 1);
    let r1_len = r1[0].len();
    let r2_len = r2[0].len();

    if r1_len > r2_len {
        return r1;
    } else if r1_len < r2_len {
        return r2;
    } else {
        r1.extend(r2);
        return r1;
    }
}

fn sort(src: &mut Vec<Vec<u8>>) {
    for i in 0..src.len() {
        for j in i + 1..src.len() {
            if src[i] > src[j] {
                src.swap(i, j);
            }
        }
    }
}

fn uniq(src: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut r = Vec::new();

    let mut last = vec![];
    for i in src {
        if i != last {
            r.push(i.clone());
        }
        last = i;
    }
    r
}


fn test() {
    println!("test");

    let a = "abcabcaa";
    let b = "acbacba";

    let mut r = find_longest_common(a.as_bytes(), 0, b.as_bytes(), 0);
    sort(&mut r);
    let r = uniq(r);
    for i in r {
        println!("{}", String::from_utf8(i).unwrap());
    }
}
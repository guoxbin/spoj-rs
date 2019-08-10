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
    let r = find_common(a.as_bytes(), 0, b.as_bytes(), 0, b.len());
    let mut r = collect(r);
    sort(&mut r);
    let r = uniq(r);
    for i in r {
        println!("{}", String::from_utf8(i).unwrap());
    }
    println!();
}

fn find_common(a: &[u8], ai: usize, b: &[u8], bi: usize, ben: usize) -> Vec<Vec<u8>> {
    if ai > a.len() - 1 || bi > b.len() - 1 {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut table = vec![0u8; 'z' as usize - 'a' as usize + 1];

    let mut jmax = ben;
    for i in ai..a.len() {
        if table[a[i] as usize - 'a' as usize] == 1u8 {
            continue;
        }

        table[a[i] as usize - 'a' as usize] = 1u8;
        for j in bi..jmax {
            if a[i] == b[j] {
                let sub_result = find_common(a, i + 1, b, j + 1, b.len());

                if sub_result.len() == 0 {
                    result.push(vec![a[i]]);
                }
                for mut one in sub_result {
                    one.insert(0, a[i]);
                    result.push(one);
                }
                jmax = j;
                break;
            }
        }
    }
    //println!("{:?} {} {:?} {} {:?}", a, ai, b, bi, result);
    result
}

fn collect(src: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut max_len = 0;
    let mut r = Vec::new();

    for i in src {
        let len = i.len();
        if len > max_len {
            max_len = len;
            r.clear();
            r.push(i);
        } else if len == max_len {
            r.push(i);
        }
    }
    r
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

    let r = find_common(a.as_bytes(), 0, b.as_bytes(), 0, b.len());

    let mut r = collect(r);

    sort(&mut r);

    let r = uniq(r);

    for i in r {
        println!("{}", String::from_utf8(i).unwrap());
    }
}
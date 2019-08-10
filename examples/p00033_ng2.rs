use std::io;
use std::env::args;
use std::io::Read;
use std::cmp::max;

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
    let map = find_longest_common_count(a.as_bytes(), b.as_bytes());
    let mut r = find_longest_common(a.as_bytes(), a.len(), b.as_bytes(), b.len(), &map);
    sort(&mut r);
    let r = uniq(r);
    for i in r {
        println!("{}", String::from_utf8(i).unwrap());
    }
    println!();
}

fn find_longest_common_count(a: &[u8], b: &[u8]) -> Vec<Vec<u32>> {
    let mut map = vec![vec![0u32; b.len() + 1]; a.len() + 1];

    for i in 0..=a.len() {
        for j in 0..=b.len() {
            if i == 0 || j == 0 {
                map[i][j] = 0;
            } else if a[i - 1] == b[j - 1] {
                map[i][j] = map[i - 1][j - 1] + 1;
            } else {
                map[i][j] = max(map[i - 1][j], map[i][j - 1]);
            }
        }
    }
    map
}

fn find_longest_common(a: &[u8], ai: usize, b: &[u8], bi: usize, map: &Vec<Vec<u32>>) -> Vec<Vec<u8>> {
    if ai == 0 || bi == 0 {
        return vec![vec![]];
    }

    if a[ai - 1] == b[bi - 1] {
        let r = find_longest_common(a, ai - 1, b, bi - 1, map);
        let mut result = Vec::new();
        for mut i in r {
            i.push(a[ai - 1]);
            result.push(i);
        }
        return result;
    }

    let r1_len = map[ai - 1][bi];
    let r2_len = map[ai][bi - 1];

    if r1_len > r2_len {
        return find_longest_common(a, ai - 1, b, bi, map);
    } else if r1_len < r2_len {
        return find_longest_common(a, ai, b, bi - 1, map);
    } else {
        let mut r1 = find_longest_common(a, ai - 1, b, bi, map);
        r1.extend(find_longest_common(a, ai, b, bi - 1, map));
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

    let map = find_longest_common_count(a.as_bytes(), b.as_bytes());
    let mut r = find_longest_common(a.as_bytes(), a.len(), b.as_bytes(), b.len(), &map);
    sort(&mut r);
    let r = uniq(r);
    for i in r {
        println!("{}", String::from_utf8(i).unwrap());
    }
}
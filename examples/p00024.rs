use std::io;
use std::env::args;

fn read_from_stdin(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

fn process(buf: &str) {}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 2 && args[1] == "test".to_string() {
        test();
        return;
    }

    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();
    let len: usize = buf.trim().parse().unwrap();

    let m = prepare();

    for _i in 0..len {
        buf.clear();
        read_from_stdin(&mut buf).unwrap();
        let n: usize = buf.trim().parse().unwrap();

        let r = m.get(n).unwrap();
        println!("{}", output_vec(r));
    }
}

fn num_to_vec(mut num: u32) -> Vec<i8> {
    let mut r = vec![];
    while num > 0 {
        r.push((num % 10) as i8);
        num = num / 10;
    }
    r
}

fn output_vec(v: &Vec<i8>) -> String {
    let mut r = String::new();
    let mut s = false;
    for i in 0..v.len() {
        let d = v[v.len() - i - 1];
        s = s || d > 0;
        if s {
            r.push((d + 48) as u8 as char);
        }
    }
    r
}

fn multiple(a: &Vec<i8>, b: &Vec<i8>) -> Vec<i8> {
    let mut r = vec![0i8; a.len() + b.len()];

    for i in 0..b.len() {
        for j in 0..a.len() {
            let mul = a[j] * b[i];
            let tj = j + i;
            r[tj] = r[tj] + mul;
            r[tj + 1] = r[tj + 1] + r[tj] / 10;
            r[tj] = r[tj] % 10;
        }
    }
    r
}

fn prepare() -> Vec<Vec<i8>> {
    let mut m = vec![vec![]; 101];
    let mut r = vec![1i8];
    for i in 1..=100 {
        r = multiple(&r, &num_to_vec(i));
        m[i as usize] = r.clone();
    }
    m
}

fn test() {
    println!("test");

    assert_eq!(vec![8, 0, 4, 0], multiple(&mut num_to_vec(12), &num_to_vec(34)));

    assert_eq!("4321", output_vec(&vec![1, 2, 3, 4]));
}
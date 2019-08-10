use std::io;
use std::env::args;
use std::io::Read;
use std::cmp::min;

const MAX: u32 = !0u32;

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
        process();
    }
}

#[derive(Debug)]
struct CoinType {
    value: u32,
    weight: u32,
    count: u32,
}

fn process() {
    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();
    let v: Vec<&str> = buf.trim().split(" ").collect();

    let pig_weight: u32 = v[0].parse().unwrap();
    let weight: u32 = v[1].parse().unwrap();

    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();

    let coin_count: u32 = buf.trim().parse().unwrap();

    let mut coin_type = Vec::new();

    for i in 0..coin_count {
        let mut buf = String::new();
        read_from_stdin(&mut buf).unwrap();
        let v: Vec<&str> = buf.trim().split(" ").collect();

        coin_type.push(CoinType {
            value: v[0].parse().unwrap(),
            weight: v[1].parse().unwrap(),
            count: 0u32,
        })
    }

    let net_weight = weight - pig_weight;

    let r = min_value(net_weight as usize, &mut coin_type);

    match r {
        Some(r) => println!("The minimum amount of money in the piggy-bank is {}.", r),
        None => println!("This is impossible."),
    }
}

fn min_value(net_weight: usize, coin_type: &mut Vec<CoinType>) -> Option<u32> {
    let mut cache : Vec<Vec<Option<u32>>> = vec![vec![None; coin_type.len() + 1]; net_weight as usize + 1];

    for i in 0..=net_weight {
        for j in 0..=coin_type.len() {
            if i == 0 || j == 0 {
                cache[i][j] = None;
            } else {
                let t = &coin_type[j - 1];
                let t_weight = t.weight as usize;
                let t_value = t.value as usize;

                let max_count = i / t_weight;

                let mut min_whole_value = MAX;

                for count in 0..=max_count {
                    let weight = count * t_weight;

                    let pre_i = i - weight;

                    let pre_j = j - 1;

                    let mut pre_min_value = 0;

                    if pre_i > 0 {

                        let a = cache[pre_i][pre_j];

                        if let Some(v) = a {
                            pre_min_value = v;
                        } else {
                            continue;
                        }
                    }

                    let whole_value = (count * t_value) as u32 + pre_min_value;


                    min_whole_value = min(min_whole_value, whole_value);
                }

                let min_whole_value = if min_whole_value != MAX { Some(min_whole_value) } else { None };

                cache[i][j] = min_whole_value;

            }
        }
    }

    cache[net_weight][coin_type.len()]
}

fn test() {
    println!("test");

    println!("{}", MAX);


    let a = min_value(100, &mut vec![
        CoinType {
            value: 1,
            weight: 1,
            count: 0,
        },
        CoinType {
            value: 30,
            weight: 50,
            count: 0,
        }
    ]);

    assert_eq!(Some(60), a);

    let a = min_value(100, &mut vec![
        CoinType {
            value: 1,
            weight: 1,
            count: 0,
        },
        CoinType {
            value: 50,
            weight: 30,
            count: 0,
        }
    ]);

    assert_eq!(Some(100), a);

    let a = min_value(5, &mut vec![
        CoinType {
            value: 10,
            weight: 3,
            count: 0,
        },
        CoinType {
            value: 20,
            weight: 4,
            count: 0,
        }
    ]);

    assert_eq!(None, a);
}
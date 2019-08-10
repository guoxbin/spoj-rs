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

    let mut cache = vec![vec![None; coin_type.len() + 1]; 10001];

    let r = min_value(net_weight, &mut coin_type, 0, &mut cache);

    match r {
        Some(r) => println!("The minimum amount of money in the piggy-bank is {}.", r),
        None => println!("This is impossible."),
    }
}

fn min_value(net_weight: u32, coin_type: &mut Vec<CoinType>, index: usize, cache: &mut Vec<Vec<Option<Option<u32>>>>) -> Option<u32> {
    let (len, t_value, t_weight) = {
        let len = coin_type.len();
        let t = &coin_type[index];
        let t_value = t.value;
        let t_weight = t.weight;
        (len, t_value, t_weight)
    };

    if index == len - 1 {

//        println!("{} {}", net_weight, t_weight);

        if net_weight % t_weight == 0 {
            let count = net_weight / t_weight;
            let value = count * t_value;
//            println!("- {} {} {} {}", net_weight, t_weight, t_value, value);
            return Some(value);
        } else {
            return None;
        }
    }

    let max_count = net_weight / t_weight;

    let mut min_whole_value = MAX;

    for count in 0..=max_count {
        let weight = count * t_weight;

        let left_weight = net_weight - weight;

        let mut left_min_value = 0;

        if left_weight > 0 {

            let a =  cache[left_weight as usize][index + 1].map(|x|{
                //println!("hit");
                x
            }).unwrap_or( {

                let m = min_value(left_weight, coin_type, index + 1, cache);
                cache[left_weight as usize][index + 1] = Some(m);
                m

            } );

            if let Some(v) = a {
                left_min_value = v;
            } else {
                continue;
            }
        }

        let whole_value = count * t_value + left_min_value;

        min_whole_value = min(min_whole_value, whole_value);
    }

    if min_whole_value != MAX { Some(min_whole_value) } else { None }
}

fn test() {
    println!("test");

    println!("{}", MAX);

    let mut cache = vec![vec![None; 3]; 10001];

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
    ], 0, &mut cache);

    assert_eq!(Some(60), a);

    let mut cache = vec![vec![None; 3]; 10001];

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
    ], 0, &mut cache);

    assert_eq!(Some(100), a);

    let mut cache = vec![vec![None; 3]; 10001];

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
    ], 0, &mut cache);

    assert_eq!(None, a);
}
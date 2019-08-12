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

    loop {
        let over = process();
        if over {
            break;
        }
    }
}

#[derive(Debug)]
struct Party {
    fee: usize,
    fun: usize,
}

fn process() -> bool {
    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();
    if buf.trim().len() == 0 {
        buf.clear();
        read_from_stdin(&mut buf).unwrap();
    }
    let a: Vec<&str> = buf.trim().split(" ").collect();

    let budget: u32 = a[0].parse().unwrap();
    let party_count: u32 = a[1].parse().unwrap();

    if budget == 0 && party_count == 0 {
        return true;
    }

    let mut party_list = Vec::new();
    for i in 0..party_count {
        let mut buf = String::new();
        read_from_stdin(&mut buf).unwrap();
        let a: Vec<&str> = buf.trim().split(" ").collect();

        party_list.push(Party {
            fee: a[0].parse().unwrap(),
            fun: a[1].parse().unwrap(),
        });
    }

    //println!("{} {} {:?}", budget, party_count, party_list);

    let m = most_fun(budget as usize, &party_list);

    println!("{} {}", m.1, m.2);

    false
}

//return (budget, whole_fee, fun)
fn most_fun(budget: usize, party_list: &Vec<Party>) -> (usize, usize, usize) {
    let mut cache: Vec<Vec<(usize, usize, usize)>> = vec![vec![(0, 0, 0); party_list.len() + 1]; budget as usize + 1];

    for i in 0..=budget {
        for j in 0..=party_list.len() {
            if i == 0 || j == 0 {
                cache[i][j] = (0, 0, 0);
            } else {

                //participate
                let party = &party_list[j - 1];
                let fee = party.fee;
                let fun = party.fun;

                if i >= fee {
                    let pre_budget = i - fee;
                    let (pre_budget, pre_whole_fee, pre_fun) = cache[pre_budget][j - 1];

                    let participate_fun = (i, pre_whole_fee + fee, pre_fun + fun);

                    let not_participate_fun = cache[i][j - 1];

                    cache[i][j] = if participate_fun.2 > not_participate_fun.2 {
                        participate_fun
                    } else if participate_fun.2 < not_participate_fun.2 {
                        not_participate_fun
                    } else {
                        if participate_fun.1 > not_participate_fun.1 {
                            not_participate_fun
                        } else {
                            participate_fun
                        }
                    };
                } else {

                    //not participate
                    let not_participate_fun = cache[i][j - 1];
                    cache[i][j] = not_participate_fun;
                }
            }
        }
    }

    cache[budget][party_list.len()]
}

fn test() {
    println!("test");

    println!("{}", MAX);

    assert_eq!((50, 49, 26), most_fun(50, &vec![Party { fee: 12, fun: 3 }, Party { fee: 15, fun: 8 }, Party { fee: 16, fun: 9 }, Party { fee: 16, fun: 6 }, Party { fee: 10, fun: 2 }, Party { fee: 21, fun: 9 }, Party { fee: 18, fun: 4 }, Party { fee: 12, fun: 4 }, Party { fee: 17, fun: 8 }, Party { fee: 18, fun: 9 }]));

    assert_eq!((50, 48, 32), most_fun(50, &vec![Party { fee: 13, fun: 8 }, Party { fee: 19, fun: 10 }, Party { fee: 16, fun: 8 }, Party { fee: 12, fun: 9 }, Party { fee: 10, fun: 2 }, Party { fee: 12, fun: 8 }, Party { fee: 13, fun: 5 }, Party { fee: 15, fun: 5 }, Party { fee: 11, fun: 7 }, Party { fee: 16, fun: 2 }]));
}
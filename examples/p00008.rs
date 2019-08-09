use std::io;

fn read_from_stdin(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

fn process(buf: &str) {
    let a: Vec<&str> = buf.split(" ").collect();
    let size = a[0].parse().unwrap();
    let count = a[1].parse().unwrap();

    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();
    let trimed = buf.trim();

    let a: Vec<&str> = trimed.split(" ").collect();
    let mut sequence: Vec<i32> = Vec::new();
    for i in 0..size {
        sequence.push(a[i].parse().unwrap());
    }

    let result = fill(sequence, count);

    let mut s = String::new();
    for i in result {
        s.push_str(&format!("{} ", i));
    }
    s = s[0..s.len() - 1].to_string();

    println!("{}", s);
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

//    test();
}

fn fill(sequence: Vec<i32>, count: u32) -> Vec<i32> {
    let mut stack = Vec::new();

    let mut current = sequence;

    loop {
        let len = current.len();

        let mut new_sequence = Vec::new();
        let mut all_zero = true;
        for i in 0..(len - 1) {
            let item = current[i + 1] - current[i];
            new_sequence.push(item);

            all_zero = all_zero && item == 0;
        }

        stack.push(current);

        if all_zero {
            break;
        }

        current = new_sequence;
    }
//    println!("{:?}", stack);

    let mut index = 0;
    loop {
        let mut a = stack.pop().unwrap();

        if index == 0 {
            let first = *a.first().unwrap();
            for i in 0..count {
                a.push(first);
            }
        }

        if stack.len() == 0 {
            stack.push(a);
            break;
        }

        let mut top = stack.last_mut().unwrap();
        let top_len = top.len();

        for i in 0..count {
            let v1 = top[top_len - 1 + i as usize];
            let v2 = a[top_len - 1 + i as usize];
            top.push(v1 + v2);
        }

        index = index + 1;
    }

    let r = &stack[0];
    let len = r.len();
    let r = &r[(len - count as usize)..];

    r.to_vec()
}

fn test() {
    assert_eq!(vec![7, 8, 9], fill(vec![1, 2, 3, 4, 5, 6], 3));

    assert_eq!(vec![37, 46], fill(vec![1, 2, 4, 7, 11, 16, 22, 29], 2));

    assert_eq!(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3], fill(vec![3], 10));

    assert_eq!(vec![1587, 7915], fill(vec![1, 5, 5, 1, 50, 148, 197, 295], 2));

    assert_eq!(vec![11, 56], fill(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 2], 2));

    assert_eq!(vec![7, 0], fill(vec![5, 6, 8, 9], 2));
}
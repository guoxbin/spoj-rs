use std::io;

fn read_from_stdin(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

fn main() {
    let mut buf = String::new();
    loop {
        buf.clear();
        let len = read_from_stdin(&mut buf).unwrap();
        if len == 0 {
            break;
        }
        let trimed = buf.trim_end();
        if trimed == "42"{
            break;
        }
        println!("{}", trimed);
    }
}
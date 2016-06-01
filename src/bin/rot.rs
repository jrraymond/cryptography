use std::env;
use std::io;
use std::io::BufRead;

fn rot(s : u8, c : u8) -> u8 {
    let (n, l, u) = match c {
        b'a'...b'z' => (c + s, b'a', b'z'),
        b'A'...b'Z' => (c + s, b'A', b'Z'),
        _ => (c, 0, 0)
    };
    if n > u {
        return n % u + l - 1;
    } else {
        return n
    }
}


fn main() {
    let mut shift = 13;
    for arg in env::args() {
        match arg.parse::<u8>() {
            Ok(x) => shift = x,
            _ => continue,
        }
        break;
    }

    let mut buffer = String::new();
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        match line {
            Ok(x) =>
            {
                for b in x.as_bytes() {
                    buffer.push(rot(shift, *b) as char);
                }
            },
            Err(_) => (),
        }
    }
    println!("{}", buffer);
}

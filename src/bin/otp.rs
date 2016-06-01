use std::io::{self, BufReader};
use std::io::prelude::*;

/* DOES NOT WORK
 *
 * a    t    t    a    c    k       a    t       d    a    w    n   message
 * 97 116   116   97   99   107 32  97  16  32  100  97   199  110  plaintext
 * 09   e1   c5   f7   0a   65  ac  51   94 58   e7  e5    3f   36  ciphertext
 */
fn get_key(plain: &str, cipher: &str) -> Vec<u8> {
    assert_eq!(2*plain.len(), cipher.len());
    let mut cipher_text = Vec::with_capacity(plain.len());
    let mut last = None;
    for ch in cipher.chars() {
        match last {
            None => last = Some(ch),
            Some(c) =>
            {
                cipher_text.push(hex_to_dec(c, ch));
                last = None;
            },
        }
    }
    let mut k = Vec::with_capacity(plain.len());
    for (p, c) in plain.as_bytes().iter().zip(cipher_text.iter()) {
        k.push(p^c);
    }
    return k;
}

fn hex_to_dec(a: char, b: char) -> u8 {
    return (a.to_digit(16).unwrap()*16 + b.to_digit(16).unwrap()) as  u8;
}

fn get_key_driver() -> io::Result<Vec<u8>> {
    let mut plain_text = String::new();
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut reader = BufReader::new(handle);

    println!("Enter plain text");
    try!(reader.read_line(&mut plain_text));

    println!("Enter cipher text");
    let mut cipher_text = String::with_capacity(2*plain_text.len());
    try!(reader.read_line(&mut cipher_text));

    return Result::Ok(get_key(plain_text.trim(), cipher_text.trim()));
}


fn opt(plain: &str, key: &Vec<u8>) -> String {
    let mut s = String::with_capacity(plain.len());
    for (c, k) in plain.chars().zip(key.iter()) {
        let hex = c as u8 ^ k;
        let ch = if hex < 16 {
            format!("0{:x}", hex)
        } else {
            format!("{:x}", hex)
        };
        s.push_str(&ch);
    }
    return s;
}

fn opt_driver(key: &Vec<u8>) {
    let mut plain_text = String::new();
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut reader = BufReader::new(handle);
    loop {
        println!("Enter plain text");
        let l = reader.read_line(&mut plain_text);
        if l.is_err() || l.unwrap() <= 0 {
            return;
        }
        println!("{}", opt(&plain_text, key));
        plain_text.clear();
    }
}


fn main() {
    let k = get_key_driver().unwrap();
    println!("{:?}", &k);
    opt_driver(&k);
}

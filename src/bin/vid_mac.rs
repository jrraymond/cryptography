extern crate cryptography;
extern crate find_folder;

use cryptography::vid_mac;

use std::fs::File;
use std::io::prelude::*;


#[allow(dead_code)]
fn as_u8(s: &str) -> Vec<u8> {
    let mut v = Vec::with_capacity(s.len()/2);
    let mut chars = s.chars();
    while let Some(a) = chars.next() {
        match chars.next() {
            None => panic!("as u8"),
            Some(b) => v.push((a.to_digit(16).unwrap() * 16 + b.to_digit(16).unwrap()) as u8),
        }
    }
    return v;
}

fn from_u8(bs: &[u8]) -> String {
    let mut result = String::with_capacity(bs.len());
    for b in bs {
        let s = if *b < 16 {
            format!("0{:x}", b)
        } else {
            format!("{:x}", b)
        };
        result.push_str(&s);
    }
    return result;
}

fn main() {
    let path = find_folder::Search::ParentsThenKids(1,1).for_folder("test_vid_0.mp4").unwrap();
    println!("{:?}",path);
    let f = File::open(&path).unwrap();
    let bs: Vec<u8> = f.bytes().map(|x| x.unwrap()).collect();
    let h = vid_mac::vmac(1024, &bs);
    println!("{:?}", from_u8(&h));
    let path = find_folder::Search::ParentsThenKids(1,1).for_folder("test_vid_1.mp4").unwrap();
    println!("{:?}",path);
    let f = File::open(&path).unwrap();
    let bs: Vec<u8> = f.bytes().map(|x| x.unwrap()).collect();
    let h = vid_mac::vmac(1024, &bs);
    println!("{:?}", from_u8(&h));
}

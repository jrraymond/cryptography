extern crate cryptography;
use cryptography::aes::*;

static K1: &'static str = "140b41b22a29beb4061bda66b6747e14";
static C1: &'static str =
    "4ca00ff4c898d61e1edbf1800618fb2828a226d160dad07883d04e008a789\
    7ee2e4b7465d5290d0c0e6c6822236e1daafb94ffe0c5da05d9476be028ad7c1d81";

static K2: &'static str = "140b41b22a29beb4061bda66b6747e14";
static C2: &'static str =
    "5b68629feb8606f9a6667670b75b38a5b4832d0f26e1ab7da33249\
    de7d4afc48e713ac646ace36e872ad5fb8a512428a6e21364b0c374df45503473c5242a253";

static K3: &'static str = "36f18357be4dbd77f050515c73fcf9f2";
static C3: &'static str =
    "69dda8455c7dd4254bf353b773304eec0ec7702330098ce7f7520d1cbbb20fc388d1b0adb50\
    54dbd7370849dbf0b88d393f252e764f1f5f7ad97ef79d59ce29f5f51eeca32eabedd9afa9329";

static K4: &'static str = "36f18357be4dbd77f050515c73fcf9f2";
static C4: &'static str =
    "770b80259ec33beb2561358a9f2dc617e46218c0a53cbeca695ae45faa8952aa0e311bde9d4\
    e01726d3184c34451";

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

fn main() {
    let cbcs = [(K1,C1), (K2,C2)];
    for &(ks,cs) in &cbcs {
        let k = as_u8(ks);
        let c = as_u8(cs);
        let mut iv = [0; 16];
        iv.clone_from_slice(&c[0..16]);

        let p = decrypt_cbc(&k, &c);
        let pt = std::str::from_utf8(&p);
        //println!("key: {:?}\ncipher text:\n{:?}\nidempotent:\n{:?}\nplain text:\n{:?}\n{:?}", k, c, encrypt_cbc(&iv,&k, &p), p, pt);
        println!("{:?}", &pt);
    }
    let ctrs = [(K3,C3),(K4,C4)];
    for &(ks,cs) in &ctrs {
        let k = as_u8(ks);
        let c = as_u8(cs);
        let mut iv = [0; 16];
        iv.clone_from_slice(&c[0..16]);

        let p = decrypt_ctr(&k, &c);
        let pt = std::str::from_utf8(&p);
        //println!("key: {:?}\ncipher text:\n{:?}\nidempotent:\n{:?}\nplain text:\n{:?}\n{:?}", k, c, encrypt_ctr(&iv,&k, &p), p, pt);
        println!("{:?}", &pt);
    }
}

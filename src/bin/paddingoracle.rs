extern crate hyper;

use hyper::Client;

const BLOCK_LEN: usize = 16;

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

fn guess(client: &Client, url: &str, ct: &[u8]) -> hyper::Result<bool> {
    use hyper::status::StatusCode;
    let s =
    {
        let mut t = String::from(url);
        t.push_str(&from_u8(&ct));
        t
    };
    let response = try!(client.get(&s).send());
    return match response.status {
        StatusCode::NotFound | StatusCode::Ok => Ok(true),
        _ => Ok(false),
    };
}

fn attack_block(client: &Client, url: &str, cipher_text: &[u8], i: usize) -> hyper::Result<Vec<u8>> {
    let start = i*BLOCK_LEN;
    let end = start + BLOCK_LEN;
    let prev = start - BLOCK_LEN;
    let mut block = Vec::with_capacity(BLOCK_LEN);
    let mut ct = cipher_text.iter().take(end).cloned().collect::<Vec<u8>>();
    for j in (0..BLOCK_LEN).rev() {
        assert_eq!(block.len(), BLOCK_LEN-j-1);
        let pad = (BLOCK_LEN-j) as u8;
        for z in 0..255 {
            ct[prev+j] = cipher_text[prev + j] ^ z ^ pad;
            for (k, b) in (j+1..BLOCK_LEN).rev().zip(block.iter()) {
                ct[prev + k] = cipher_text[prev + k] ^ b ^ pad;
            }
            if try!(guess(client, url, &ct)) && z != 1 {
                block.push(z);
                break;
            }
        }
    }
    block.reverse();
    return Ok(block);
}

fn padding_attack(url: &str, cipher_text: &[u8]) -> hyper::Result<Vec<u8>> {
    let client = Client::new();
    let mut res = Vec::with_capacity(cipher_text.len()-BLOCK_LEN);
    let last = cipher_text.len()/BLOCK_LEN;
    for i in 1..last {
        let block = try!(attack_block(&client, url, cipher_text, i));
        println!("{:?}", std::str::from_utf8(&block));
        res.extend(block);
    }
    let padding = res.len() - *res.last().unwrap() as usize;
    res.truncate(padding);
    return Ok(res);
}

fn quiz() {
    // first 16 bytes are IV
    // next 12 bytes are message
    // last 4 bytes are padding
    // change 100 to 500 by changing byte 8 of IV to IV_8 ^ 1 ^ 5
    // 0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15
    //
    // 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
    // P  a  y     B  o  b     1  0  0  $  4  4  4  4
    let mut ct =
    {
        let t = "20814804c1767293b99f1d9cab3bc3e7ac1e37bfb15599e5f40eef805488281d";
        as_u8(&t)
    };
    let pt = "Pay Bob 100$".bytes().collect::<Vec<u8>>();
    println!("{},{}",ct.len(),pt.len());
    ct[8] ^= 1 ^ 5;
    println!("{}", from_u8(&ct));
}

fn main() {
    quiz();
    let url = "http://crypto-class.appspot.com/po?er=";
    let cipher_text =
    {
        let t = "f20bdba6ff29eed7b046d1df9fb7000058b1ffb4210a580f748\
                 b4ac714c001bd4a61044426fb515dad3f21f18aa577c0bdf302\
                 936266926ff37dbf7035d5eeb4";
        as_u8(&t)
    };

    match padding_attack(&url, &cipher_text) {
       Result::Err(e) => println!("{:?}", e),
       Result::Ok(v) => println!("{:?}", std::str::from_utf8(&v)),
    }
}

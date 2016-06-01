extern crate rand;

use std::str;
use std::collections::HashSet;
use std::collections::HashMap;


#[allow(dead_code)]
static COUNT_THRESHOLD: f64 = 1.0;

fn xor(a: &[u8], b:&[u8]) -> Vec<u8> {
    return a.iter().zip(b.iter()).map(|(&c, &d)| c ^ d).collect();
}

#[allow(dead_code)]
fn is_english(word: &[u8], dict: &HashSet<Vec<u8>>) -> bool {
    return dict.contains(word);
}

#[allow(dead_code)]
fn grab_crib(word: &[u8], cipher: &[u8], cipher_texts: &[Vec<u8>], dict: &HashSet<Vec<u8>>) -> Option<(usize,usize)> {
    let xors: Vec<Vec<u8>> = cipher_texts.iter().map(|s| xor(cipher, s)).collect(); //TODO precompute this shit
    for i in 0..cipher.len()-word.len() {
        let mut count = 0.0;
        for c in &xors {
            let result = xor(word, c);
            if is_english(&result, dict) {
                count += 1.0;
            }
        }
        if count/(xors.len() as f64) >= COUNT_THRESHOLD/(xors.len() as f64) {
            return Some((i, i + word.len()));
        }
    }
    return None;
}

fn as_u8(s: &str) -> Vec<u8> {
    let mut v = Vec::with_capacity(s.len()/2);
    let mut chars = s.chars();
    while let Some(a) = chars.next() {
        match chars.next() {
            None => panic!("us u8"),
            Some(b) => v.push((a.to_digit(16).unwrap() * 16 + b.to_digit(16).unwrap()) as u8),
        }
    }
    return v;
}

#[allow(dead_code)]
fn find_spaces(s: &Vec<u8>, cts: &Vec<Vec<u8>>) -> Vec<u8>{
    let mut table = vec![0; s.len()];
    for (ix, c) in s.iter().enumerate() {
        let mut count = 0;
        for ct in cts {
            if ix >= ct.len() {
                continue;
            }
            if ((c ^ ct[ix]) as char).is_alphabetic() {
                count += 1;
            }
        }
        if count > cts.len()-3 {
            table[ix] = ' ' as u8;
        }
    }
    return table;
}

#[allow(dead_code)]
fn all_xors(cts: &Vec<Vec<u8>>) -> HashMap<(usize, usize), Vec<u8>> {
    let mut ht = HashMap::with_capacity(cts.len().pow(2)/2);
    for i in 0..cts.len()-1 {
        for j in i+1..cts.len() {
            ht.insert((i,j), xor(&cts[i], &cts[j]));
        }
    }
    return ht;
}

#[allow(dead_code)]
fn ascii_xors() -> HashMap<u8, HashSet<(u8,u8)>> {
    let mut ht = HashMap::with_capacity(256*256);
    for i in 0..std::u8::MAX-1 {
        for j in i+1..std::u8::MAX {
            let x = i^j;
            ht.entry(x).or_insert_with(|| HashSet::with_capacity(4)).insert((i,j));
        }
    }
    return ht;
}

fn decrypt(cts: &Vec<Vec<u8>>) -> Vec<Vec<u8>>{
    let mut pts: Vec<Vec<u8>> = Vec::with_capacity(cts.len());
    for (j,ct) in cts.iter().enumerate() {
        let mut pt = Vec::with_capacity(ct.len());
        for i in 0..ct.len() {
            let mut freq: [f64;256] = [0.0; 256];
            for (k,ct2) in cts.iter().enumerate() {
                if j == k || i >= ct2.len() {
                    continue;
                }
                let x = (ct[i]^ct2[i]) as char;
                if x.is_alphabetic() {
                    freq[' ' as u8 as usize] += 0.49;
                    if x.is_uppercase() {
                        freq[x.to_lowercase().next().unwrap() as usize] += 1.0;
                    } else if x.is_lowercase() {
                        freq[x.to_uppercase().next().unwrap() as usize] += 1.0;
                    }
                }
            }
            let mut mx_f = 0.0;
            let mut ch = 0;
            for (c, &f) in freq.iter().enumerate() {
                if f > mx_f {
                    mx_f = f;
                    ch = c;
                }
            }
            assert!(ch <= std::u8::MAX as usize);
            pt.push(ch as u8);
        }
        pts.push(pt);
    }
    return pts;
}

fn show_results(v: &Vec<u8>) -> String {
    return v.iter().map(|&b| b as char).collect();
}


fn main() {
    let cipher_texts = [CT1, CT2, CT3, CT4, CT5, CT6, CT7, CT8, CT9, CT10, CT11].iter().map(|t| as_u8(t)).collect();
    let decrypted = decrypt(&cipher_texts);
    for (_, dt) in cipher_texts.iter().zip(decrypted.iter()) {
        println!("{:?}", show_results(&dt));
    }
    //test()
}

fn mk_key(sz: usize) -> Vec<u8> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    return (0..sz).map(|_| rng.gen::<u8>()).collect();
}

fn otp(msg: &[u8], key: &[u8]) -> Vec<u8> {
    return msg.iter().zip(key.iter()).map(|(&a, &b)| a ^ b).collect();
}

#[allow(dead_code)]
fn gen_test_ciphers() -> (Vec<u8>,Vec<Vec<u8>>) {
    let msg = "computer science is the scientific and practical approach to computation and its applications \
              it is the systematic study of the feasibility, structure, expression, and mechanization of the \
              methodical procedures (or algorithms) that underlie the acquisition, representation, processing\
              storage, communication of, and access to information. An alternate definition is the study of \
              automating algorithmic processes that scale. Godel, Escher, Bach: An Eternal Golden Braid \
              is a metaphorical fugue on minds and machines in the spirit of Lewis Carroll. By exploring on \
              common thems in the lives and work of a logician, an artist, and a composer, the book \
              expounds concepts fundamental to mathematics, symmetry, and intelligence. The book discusses \
              how self-reference and formal rules allow systems to acquire meaning despite being made of \
              meaningless elements. It also discusses what it means to communicate, how knowledge can be \
              represented and stored, the methods and limitation of symbolic representation, and even \
              the fundamental notion of meaning itself. The book is not about the relationship between \
              mathematics, art, and music, but rather about how cognition emerges from hidden neurological \
              mechanisms";
    //let msg = "this    ";
    let key = mk_key(40);
    let mut cipher_texts = Vec::with_capacity(32);
    let mut i = 0;
    while i < msg.len() {
        let j = std::cmp::min(i+key.len(), msg.len());
        cipher_texts.push(otp(msg[i..j].as_bytes(), &key));
        i += key.len();
    }
    return (key, cipher_texts);
}

fn get_score(plain_text: &[u8], decrypted: &[u8]) -> f64 {
    assert_eq!(plain_text.len(), decrypted.len());
    let mut s = 0.0;
    for (&p, &d) in plain_text.iter().zip(decrypted.iter()) {
        if p == d {
            s += 1.0;
        }
    }
    return s/(plain_text.len() as f64);
}

#[allow(dead_code)]
fn test() {
    let (key, cipher_texts) = gen_test_ciphers();
    let decrypted = decrypt(&cipher_texts);
    for (ct, dt) in cipher_texts.iter().zip(decrypted.iter()) {
        let pt = otp(&ct, &key);
        let s = get_score(&pt, &dt);
        println!("{:?}\n{:?}\n{:?}\n{:?}\n{}", str::from_utf8(&pt), str::from_utf8(&dt), &pt, &dt, s);
    }
}


//ciphertext #1:

static CT1: &'static str = "315c4eeaa8b5f8aaf9174145bf43e1784b8fa00dc71d885a804e5ee9fa40b\
                             16349c146fb778cdf2d3aff021dfff5b403b510d0d0455468aeb98622b137\
                             dae857553ccd8883a7bc37520e06e515d22c954eba5025b8cc57ee59418c\
                             e7dc6bc41556bdb36bbca3e8774301fbcaa3b83b220809560987815f65286\
                             764703de0f3d524400a19b159610b11ef3e";

//ciphertext #2:

static CT2: &'static str = "234c02ecbbfbafa3ed18510abd11fa724fcda2018a1a8342cf064bbde548b12b07df44ba7\
191d9606ef4081ffde5ad46a5069d9f7f543bedb9c861bf29c7e205132eda9382b0bc2c5c4b45f919cf3a9f1cb74151f6d55\
1f4480c82b2cb24cc5b028aa76eb7b4ab24171ab3cdadb8356f";

//ciphertext #3:

static CT3: &'static str = "32510ba9a7b2bba9b8005d43a304b5714cc0bb0c8a34884dd91304b8ad40b62b07df44ba6e\
9d8a2368e51d04e0e7b207b70b9b8261112bacb6c866a232dfe257527dc29398f5f3251a0d47e503c66e935de81230b59b7a\
fb5f41afa8d661cb";

//ciphertext #4:

static CT4: &'static str = "32510ba9aab2a8a4fd06414fb517b5605cc0aa0dc91a8908c2064ba8ad5ea06a029056f47a\
8ad3306ef5021eafe1ac01a81197847a5c68a1b78769a37bc8f4575432c198ccb4ef63590256e305cd3a9544ee4160ead45a\
ef520489e7da7d835402bca670bda8eb775200b8dabbba246b130f040d8ec6447e2c767f3d30ed81ea2e4c1404e1315a1010\
e7229be6636aaa";

//ciphertext #5:

static CT5: &'static str = "3f561ba9adb4b6ebec54424ba317b564418fac0dd35f8c08d31a1fe9e24fe56808c213f17c\
81d9607cee021dafe1e001b21ade877a5e68bea88d61b93ac5ee0d562e8e9582f5ef375f0a4ae20ed86e935de81230b59b73\
fb4302cd95d770c65b40aaa065f2a5e33a5a0bb5dcaba43722130f042f8ec85b7c2070";

//ciphertext #6:

static CT6: &'static str = "32510bfbacfbb9befd54415da243e1695ecabd58c519cd4bd2061bbde24eb76a19d84aba34\
d8de287be84d07e7e9a30ee714979c7e1123a8bd9822a33ecaf512472e8e8f8db3f9635c1949e640c621854eba0d79eccf52\
ff111284b4cc61d11902aebc66f2b2e436434eacc0aba938220b084800c2ca4e693522643573b2c4ce35050b0cf774201f0f\
e52ac9f26d71b6cf61a711cc229f77ace7aa88a2f19983122b11be87a59c355d25f8e4";

//ciphertext #7:

static CT7: &'static str = "32510bfbacfbb9befd54415da243e1695ecabd58c519cd4bd90f1fa6ea5ba47b01c909ba76\
96cf606ef40c04afe1ac0aa8148dd066592ded9f8774b529c7ea125d298e8883f5e9305f4b44f915cb2bd05af51373fd9b4a\
f511039fa2d96f83414aaaf261bda2e97b170fb5cce2a53e675c154c0d9681596934777e2275b381ce2e40582afe67650b13\
e72287ff2270abcf73bb028932836fbdecfecee0a3b894473c1bbeb6b4913a536ce4f9b13f1efff71ea313c8661dd9a4ce";

//ciphertext #8:

static CT8: &'static str = "315c4eeaa8b5f8bffd11155ea506b56041c6a00c8a08854dd21a4bbde54ce56801d943ba70\
8b8a3574f40c00fff9e00fa1439fd0654327a3bfc860b92f89ee04132ecb9298f5fd2d5e4b45e40ecc3b9d59e9417df7c95b\
ba410e9aa2ca24c5474da2f276baa3ac325918b2daada43d6712150441c2e04f6565517f317da9d3";

//ciphertext #9:

static CT9: &'static str = "271946f9bbb2aeadec111841a81abc300ecaa01bd8069d5cc91005e9fe4aad6e04d513e96d\
99de2569bc5e50eeeca709b50a8a987f4264edb6896fb537d0a716132ddc938fb0f836480e06ed0fcd6e9759f40462f9cf57\
f4564186a2c1778f1543efa270bda5e933421cbe88a4a52222190f471e9bd15f652b653b7071aec59a2705081ffe72651d08\
f822c9ed6d76e48b63ab15d0208573a7eef027";

//ciphertext #10:

static CT10: &'static str = "466d06ece998b7a2fb1d464fed2ced7641ddaa3cc31c9941cf110abbf409ed39598005b339\
9ccfafb61d0315fca0a314be138a9f32503bedac8067f03adbf3575c3b8edc9ba7f537530541ab0f9f3cd04ff50d66f1d559\
ba520e89a2cb2a83";

//target ciphertext (decrypt this one):

static CT11: &'static str = "32510ba9babebbbefd001547a810e67149caee11d945cd7fc81a05e9f85aac650e9052ba6a\
8cd8257bf14d13e6f0a803b54fde9e77472dbff89d71b57bddef121336cb85ccb8f3315f4b52e301d16e9f52f904";

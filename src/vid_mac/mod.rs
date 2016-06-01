use crypto::sha2::Sha256;
use crypto::digest::Digest;

const HASH_SZ: usize = 32;

pub fn vmac(block_sz: usize, inp: &[u8]) -> Vec<u8> {
    let mut sha = Sha256::new();
    let mut i = (inp.len()/block_sz)*block_sz;

    let mut buffer = Vec::with_capacity(block_sz + HASH_SZ);
    unsafe {
        buffer.set_len(block_sz + HASH_SZ);
    }
    sha.input(&inp[i..inp.len()]);
    sha.result(&mut buffer[block_sz..]);


    while i > 0 {
        i -= block_sz;
        &mut buffer[..block_sz].clone_from_slice(&inp[i..i+block_sz]);
        sha.reset();
        sha.input(&buffer);
        sha.result(&mut buffer[block_sz..]);
    }
    let mut h = Vec::with_capacity(HASH_SZ);
    h.extend(&buffer[block_sz..]);
    return h;
}

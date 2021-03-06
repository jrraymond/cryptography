pub const BLOCK_LEN: usize = 16;
pub const BLOCK_DIM: usize = 4;
pub const ROUNDS: usize = 10;
pub const NK: usize = 4;

pub fn padding(key_len: usize, text_len: usize) -> usize {
    let x = ((text_len as f32 + 0.5) / (key_len as f32)).ceil() as usize;
    return x * key_len - text_len;
}

pub fn row_to_col_major(inp: &[u8], mut out: &mut [u8]) {
    for r in 0..BLOCK_DIM {
        for c in 0..BLOCK_DIM {
            out[r + BLOCK_DIM * c] = inp[BLOCK_DIM * r + c];
        }
    }
}

pub fn col_to_row_major(inp: &[u8], mut out: &mut [u8]) {
    for r in 0..BLOCK_DIM {
        for c in 0..BLOCK_DIM {
            out[BLOCK_DIM * r + c] = inp[r + BLOCK_DIM * c];
        }
    }
}

pub const SBOX: [u8; 256] =
    [0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76,
     0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0,
     0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15,
     0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75,
     0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84,
     0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf,
     0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8,
     0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2,
     0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73,
     0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb,
     0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79,
     0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08,
     0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a,
     0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e,
     0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf,
     0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16,
    ];

pub const INV_SBOX: [u8; 256] =
	[0x52,0x09,0x6a,0xd5,0x30,0x36,0xa5,0x38,0xbf,0x40,0xa3,0x9e,0x81,0xf3,0xd7,0xfb,
  	 0x7c,0xe3,0x39,0x82,0x9b,0x2f,0xff,0x87,0x34,0x8e,0x43,0x44,0xc4,0xde,0xe9,0xcb,
  	 0x54,0x7b,0x94,0x32,0xa6,0xc2,0x23,0x3d,0xee,0x4c,0x95,0x0b,0x42,0xfa,0xc3,0x4e,
  	 0x08,0x2e,0xa1,0x66,0x28,0xd9,0x24,0xb2,0x76,0x5b,0xa2,0x49,0x6d,0x8b,0xd1,0x25,
  	 0x72,0xf8,0xf6,0x64,0x86,0x68,0x98,0x16,0xd4,0xa4,0x5c,0xcc,0x5d,0x65,0xb6,0x92,
  	 0x6c,0x70,0x48,0x50,0xfd,0xed,0xb9,0xda,0x5e,0x15,0x46,0x57,0xa7,0x8d,0x9d,0x84,
  	 0x90,0xd8,0xab,0x00,0x8c,0xbc,0xd3,0x0a,0xf7,0xe4,0x58,0x05,0xb8,0xb3,0x45,0x06,
  	 0xd0,0x2c,0x1e,0x8f,0xca,0x3f,0x0f,0x02,0xc1,0xaf,0xbd,0x03,0x01,0x13,0x8a,0x6b,
  	 0x3a,0x91,0x11,0x41,0x4f,0x67,0xdc,0xea,0x97,0xf2,0xcf,0xce,0xf0,0xb4,0xe6,0x73,
  	 0x96,0xac,0x74,0x22,0xe7,0xad,0x35,0x85,0xe2,0xf9,0x37,0xe8,0x1c,0x75,0xdf,0x6e,
  	 0x47,0xf1,0x1a,0x71,0x1d,0x29,0xc5,0x89,0x6f,0xb7,0x62,0x0e,0xaa,0x18,0xbe,0x1b,
  	 0xfc,0x56,0x3e,0x4b,0xc6,0xd2,0x79,0x20,0x9a,0xdb,0xc0,0xfe,0x78,0xcd,0x5a,0xf4,
  	 0x1f,0xdd,0xa8,0x33,0x88,0x07,0xc7,0x31,0xb1,0x12,0x10,0x59,0x27,0x80,0xec,0x5f,
  	 0x60,0x51,0x7f,0xa9,0x19,0xb5,0x4a,0x0d,0x2d,0xe5,0x7a,0x9f,0x93,0xc9,0x9c,0xef,
  	 0xa0,0xe0,0x3b,0x4d,0xae,0x2a,0xf5,0xb0,0xc8,0xeb,0xbb,0x3c,0x83,0x53,0x99,0x61,
  	 0x17,0x2b,0x04,0x7e,0xba,0x77,0xd6,0x26,0xe1,0x69,0x14,0x63,0x55,0x21,0x0c,0x7d,
	];

// The round constant word array, Rcon[i], contains the values given by
// x to th e power (i-1) being powers of x (x is denoted as {02}) in the field GF(2^8)
// Note that i starts at 1, not 0).
pub const RCON: [u8; 255] = [
	0x8d,0x01,0x02,0x04,0x08,0x10,0x20,0x40,0x80,0x1b,0x36,0x6c,0xd8,0xab,0x4d,0x9a,
	0x2f,0x5e,0xbc,0x63,0xc6,0x97,0x35,0x6a,0xd4,0xb3,0x7d,0xfa,0xef,0xc5,0x91,0x39,
	0x72,0xe4,0xd3,0xbd,0x61,0xc2,0x9f,0x25,0x4a,0x94,0x33,0x66,0xcc,0x83,0x1d,0x3a,
	0x74,0xe8,0xcb,0x8d,0x01,0x02,0x04,0x08,0x10,0x20,0x40,0x80,0x1b,0x36,0x6c,0xd8,
	0xab,0x4d,0x9a,0x2f,0x5e,0xbc,0x63,0xc6,0x97,0x35,0x6a,0xd4,0xb3,0x7d,0xfa,0xef,
	0xc5,0x91,0x39,0x72,0xe4,0xd3,0xbd,0x61,0xc2,0x9f,0x25,0x4a,0x94,0x33,0x66,0xcc,
	0x83,0x1d,0x3a,0x74,0xe8,0xcb,0x8d,0x01,0x02,0x04,0x08,0x10,0x20,0x40,0x80,0x1b,
	0x36,0x6c,0xd8,0xab,0x4d,0x9a,0x2f,0x5e,0xbc,0x63,0xc6,0x97,0x35,0x6a,0xd4,0xb3,
	0x7d,0xfa,0xef,0xc5,0x91,0x39,0x72,0xe4,0xd3,0xbd,0x61,0xc2,0x9f,0x25,0x4a,0x94,
	0x33,0x66,0xcc,0x83,0x1d,0x3a,0x74,0xe8,0xcb,0x8d,0x01,0x02,0x04,0x08,0x10,0x20,
	0x40,0x80,0x1b,0x36,0x6c,0xd8,0xab,0x4d,0x9a,0x2f,0x5e,0xbc,0x63,0xc6,0x97,0x35,
	0x6a,0xd4,0xb3,0x7d,0xfa,0xef,0xc5,0x91,0x39,0x72,0xe4,0xd3,0xbd,0x61,0xc2,0x9f,
	0x25,0x4a,0x94,0x33,0x66,0xcc,0x83,0x1d,0x3a,0x74,0xe8,0xcb,0x8d,0x01,0x02,0x04,
	0x08,0x10,0x20,0x40,0x80,0x1b,0x36,0x6c,0xd8,0xab,0x4d,0x9a,0x2f,0x5e,0xbc,0x63,
	0xc6,0x97,0x35,0x6a,0xd4,0xb3,0x7d,0xfa,0xef,0xc5,0x91,0x39,0x72,0xe4,0xd3,0xbd,
	0x61,0xc2,0x9f,0x25,0x4a,0x94,0x33,0x66,0xcc,0x83,0x1d,0x3a,0x74,0xe8,0xcb
	];

pub fn sub_bytes(state: &mut [u8]) {
    for i in 0..BLOCK_LEN {
        let x = state[i] >> 4;
        let y = state[i] & ((1<<4) - 1);
        state[i] = SBOX[(16 * x + y) as usize];
    }
}

pub fn inv_sub_bytes(state: &mut [u8]) {
    for i in 0..BLOCK_LEN {
        let x = state[i] >> 4;
        let y = state[i] & ((1 << 4) - 1);
        state[i] = INV_SBOX[(16 * x + y) as usize];
    }
}

pub fn xtime(x: u8) -> u8 {
    (x << 1) ^ (((x >> 7) & 1) * 0x1b)
}

#[allow(non_snake_case)]
pub fn x9(x: u8) -> u8 {
    xtime(xtime(xtime(x))) ^ x
}

#[allow(non_snake_case)]
pub fn xB(x: u8) -> u8 {
    xtime(xtime(xtime(x)) ^ x) ^ x
}

#[allow(non_snake_case)]
pub fn xD(x: u8) -> u8 {
    xtime(xtime(xtime(x) ^ x)) ^ x
}

#[allow(non_snake_case)]
pub fn xE(x: u8) -> u8 {
    xtime(xtime(xtime(x) ^ x) ^ x)
}


pub fn shift_rows(state: &mut [u8]) {
    let mut old = [0; BLOCK_LEN];
    old.clone_from_slice(&state);

    state[13] = old[1];
    state[1] = old[5];
    state[5] = old[9];
    state[9] = old[13];

    state[10] = old[2];
    state[14] = old[6];
    state[2] = old[10];
    state[6] = old[14];

    state[15] = old[11];
    state[11] = old[7];
    state[7] = old[3];
    state[3] = old[15];
}

pub fn inv_shift_rows(state: &mut [u8]) {
    let mut old = [0; BLOCK_LEN];
    old.clone_from_slice(&state);

    state[1] = old[13];
    state[5] = old[1];
    state[9] = old[5];
    state[13] = old[9];

    state[10] = old[2];
    state[14] = old[6];
    state[2] = old[10];
    state[6] = old[14];

    state[3] = old[7];
    state[7] = old[11];
    state[11] = old[15];
    state[15] = old[3];
}

/* s_0c'   02 03 01 01  s0c
 * s_1c' = 01 02 03 01  s1c
 * s_2c'   01 01 02 03  s2c
 * s_3c'   01 01 01 02  s3c
 */
pub fn mix_cols(state: &mut [u8]) {
    let mut old = [0; BLOCK_LEN];
    old.clone_from_slice(&state);

    for i in 0..4 {
        let c = 4*i;
        state[c] = xtime(old[c])^xtime(old[c+1])^old[c+1]^old[c+2]^old[c+3];
        state[c+1] = old[c]^xtime(old[c+1])^xtime(old[c+2])^old[c+2]^old[c+3];
        state[c+2] = old[c]^old[c+1]^xtime(old[c+2])^xtime(old[c+3])^old[c+3];
        state[c+3] = xtime(old[c])^old[c]^old[c+1]^old[c+2]^xtime(old[c+3]);
    }
}

/* s_0c'   0e 0b 0d 09  s0c
 * s_1c' = 09 0e 0b 0d  s1c
 * s_2c'   0d 09 0e 0b  s2c
 * s_3c'   0b 0d 09 0e  s3c
 */
pub fn inv_mix_cols(state: &mut [u8]) {
    let mut old = [0; BLOCK_LEN];
    old.clone_from_slice(&state);

    for i in 0..4 {
        let c = 4*i;
        state[c] = xE(old[c])^xB(old[c+1])^xD(old[c+2])^x9(old[c+3]);
        state[c+1] = x9(old[c])^xE(old[c+1])^xB(old[c+2])^xD(old[c+3]);
        state[c+2] = xD(old[c])^x9(old[c+1])^xE(old[c+2])^xB(old[c+3]);
        state[c+3] = xB(old[c])^xD(old[c+1])^x9(old[c+2])^xE(old[c+3]);
    }
}

pub fn add_round_key(state: &mut [u8], round_keys: &[u8], i: usize) {
    let s = i * BLOCK_LEN;
    for j in 0..BLOCK_LEN {
        state[j] = state[j] ^ round_keys[s+j];
    }
}

pub fn rot_word(w: &mut [u8]) {
    let k = w[0];
    for i in 0..BLOCK_DIM-1 {
        w[i] = w[i+1];
    }
    w[BLOCK_DIM-1] = k;
}

pub fn sub_word(w: &mut [u8]) {
    for i in 0..BLOCK_DIM {
        w[i] = SBOX[w[i] as usize];
    }
}

pub fn arr_as_hex(v: &[u8]) {
    let mut buff = String::with_capacity(2*v.len()+2);
    buff.push('(');
    for (i,x) in v.iter().enumerate() {
        let s =
            if i < v.len()-1 {
                format!("{:X},", x)
            } else {
                format!("{:X})", x)
            };
        buff.push_str(&s);
    }
    println!("{}", buff);
}

pub fn expand_key(key: &[u8], round_keys: &mut [u8]) {
    let mut temp = [0; NK];

    for i in 0..BLOCK_LEN {
        round_keys[i] = key[i];
    }
    let mut i = BLOCK_LEN;
    while i < BLOCK_LEN*(ROUNDS+1) {
        temp.clone_from_slice(&round_keys[i-NK..i]);
        if i % (4*NK) == 0 {
            rot_word(&mut temp);
            sub_word(&mut temp);
			temp[0] = temp[0] ^ RCON[i/(4*NK)];
        } else if NK > 6 && i % (4*NK) == 4 {
            sub_word(&mut temp);
        }
        for j in 0..NK {
            round_keys[i+j] = round_keys[i+j-4*NK] ^ temp[j];
        }
        i += NK;
    }
}


pub fn encrypt_block(key: &[u8], inp: &[u8], out: &mut [u8]) {
    let mut state = [0; BLOCK_LEN];
    let round_keys =
    {
        let mut x = [0; (ROUNDS+1)*BLOCK_LEN];
        expand_key(key, &mut x);
        x
    };

    state.clone_from_slice(&inp);

    add_round_key(&mut state, &round_keys, 0);

    for i in 1..ROUNDS {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        mix_cols(&mut state);
        add_round_key(&mut state, &round_keys, i);
    }

    sub_bytes(&mut state);
    shift_rows(&mut state);
    add_round_key(&mut state, &round_keys, ROUNDS);

    for i in 0..state.len() {
        out[i] = state[i];
    }
}

pub fn decrypt_block(key: &[u8], inp: &[u8], out: &mut [u8]) {
    let mut state = [0; BLOCK_LEN];
    let round_keys =
    {
        let mut x = [0; (ROUNDS+1)*BLOCK_LEN];
        expand_key(&key, &mut x);
        x
    };

    state.clone_from_slice(inp);

    add_round_key(&mut state, &round_keys, ROUNDS);

    for i in (1..ROUNDS).rev() {
        inv_shift_rows(&mut state);
        inv_sub_bytes(&mut state);
        add_round_key(&mut state, &round_keys, i);
        inv_mix_cols(&mut state);
    }

    inv_shift_rows(&mut state);
    inv_sub_bytes(&mut state);
    add_round_key(&mut state, &round_keys, 0);

    for i in 0..state.len() {
        out[i] = state[i];
    }
}

pub fn encrypt_cbc(iv: &[u8], key: &[u8], plain_text: &[u8]) -> Vec<u8> {
    let pad_len = padding(key.len(), plain_text.len() + iv.len());
    let mut cipher_text = Vec::with_capacity(iv.len() + plain_text.len() + pad_len);
    cipher_text.extend(iv);

    let mut ix = 0;
    let mut prev = [0; BLOCK_LEN]; //IV TODO
    prev.clone_from_slice(&iv);
    while ix < plain_text.len() {
        let mut output = [0; BLOCK_LEN];

        for i in 0..BLOCK_LEN {
            if ix + i < plain_text.len() {
                prev[i] = prev[i] ^ plain_text[ix+i];
            } else {
                prev[i] = prev[i] ^ (pad_len as u8);
            }
        }
        encrypt_block(&key[0..16], &prev, &mut output);

        prev.clone_from_slice(&output);
        cipher_text.extend(&prev);
        ix += BLOCK_LEN;
    }

    return cipher_text;
}

pub fn decrypt_cbc(key: &[u8], cipher_text: &[u8]) -> Vec<u8> {
    let mut plain_text = Vec::with_capacity(cipher_text.len());

    let mut i = BLOCK_LEN;
    while i < cipher_text.len() {
        let mut output = [0; BLOCK_LEN];

        decrypt_block(key, &cipher_text[i..i+BLOCK_LEN], &mut output);

        for j in 0..BLOCK_LEN {
            output[j] ^= cipher_text[i+j-BLOCK_LEN];
        }
        plain_text.extend(&output);
        i += BLOCK_LEN;
    }
    let padding = plain_text[plain_text.len()-1] as usize;
    let l = plain_text.len()-padding;
    plain_text.resize(l, 0);

    return plain_text;
}

pub fn encrypt_ctr(iv: &[u8], key: &[u8], plain_text: &[u8]) -> Vec<u8> {
    let mut cipher_text = Vec::with_capacity(plain_text.len() + iv.len());

    let mut ctr = ((iv[0] as u32) << 24) | ((iv[1] as u32) << 16) | ((iv[2] as u32) << 8) | (iv[3] as u32);
    cipher_text.extend(iv);
    let mut i = 0;
    while i < plain_text.len() {
        let k = [(ctr >> 24) as u8, ((ctr >> 16) & 0x0FF) as u8, ((ctr >> 8) & 0x0FF) as u8, (ctr & 0x0FF) as u8];
        let mut output = [0; BLOCK_LEN];
        encrypt_block(key, &k, &mut output);

        for j in 0..BLOCK_LEN {
            output[i] = output[i] ^ plain_text[i+j];
        }
        cipher_text.extend(&output);
        i += BLOCK_LEN;
        ctr += 1;
    }
    return cipher_text;
}

pub fn inc(v: &mut [u8]) {
    let mut i = v.len() - 1;
    loop {
        v[i] += 1;
        if v[i] != 0 || i == 0 {
           return;
        }
        i -= 1;
    }
}

pub fn decrypt_ctr(key: &[u8], cipher_text: &[u8]) -> Vec<u8> {
    use std::cmp;
    let mut plain_text = Vec::with_capacity(cipher_text.len()-BLOCK_LEN);

    let mut iv = [0; BLOCK_LEN];
    iv.clone_from_slice(&cipher_text[0..BLOCK_LEN]);

    let mut i = BLOCK_LEN;
    while i < cipher_text.len() {
        let mut output = [0; BLOCK_LEN];
        encrypt_block(key, &iv, &mut output);
        let end = cmp::min(BLOCK_LEN, cipher_text.len()-i);

        for j in 0..end {
            output[j] ^= cipher_text[i+j];
        }
        plain_text.extend(&output[..end]);
        i += BLOCK_LEN;
        inc(&mut iv);
    }
    return plain_text;
}

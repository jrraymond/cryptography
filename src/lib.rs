extern crate crypto;
extern crate rand;

//mod multitimepad;
pub mod aes;
pub mod vid_mac;

#[cfg(test)]
pub mod aes_test {
    use super::aes::*;
    #[test]
    fn test_key_expansion() {
        use aes::*;
        let key = [0x2b,0x7e,0x15,0x16,
                   0x28,0xae,0xd2,0xa6,
                   0xab,0xf7,0x15,0x88,
                   0x09,0xcf,0x4f,0x3c];
        let w = [
            0x2b,0x7e,0x15,0x16,
            0x28,0xae,0xd2,0xa6,
            0xab,0xf7,0x15,0x88,
            0x09,0xcf,0x4f,0x3c,
            0xa0,0xfa,0xfe,0x17,
            0x88,0x54,0x2c,0xb1,
            0x23,0xa3,0x39,0x39,
            0x2a,0x6c,0x76,0x05,
        ];
        let mut expanded = [0; (ROUNDS+1)*BLOCK_LEN];
        expand_key(&key, &mut expanded);

        {
            let mut v = Vec::with_capacity(expanded.len());
            v.extend_from_slice(&expanded);
            arr_as_hex(&v);
            arr_as_hex(&w);
        }

        for i in 0..w.len() {
            assert_eq!(expanded[i], w[i]);
        }
    }
    #[test]
    fn test_sub_bytes() {
        let mut w = [
            0x19,0x3d,0xe3,0xbe,
            0xa0,0xf4,0xe2,0x2b,
            0x9a,0xc6,0x8d,0x2a,
            0xe9,0xf8,0x48,0x08,
        ];
        let a = [
            0xd4,0x27,0x11,0xae,
            0xe0,0xbf,0x98,0xf1,
            0xb8,0xb4,0x5d,0xe5,
            0x1e,0x41,0x52,0x30,
        ];
        sub_bytes(&mut w);
        for i in 0..a.len() {
            assert_eq!(a[i], w[i]);
        }
    }
    #[test]
    fn test_inv_sub_bytes() {
        let a = [
            0x19,0x3d,0xe3,0xbe,
            0xa0,0xf4,0xe2,0x2b,
            0x9a,0xc6,0x8d,0x2a,
            0xe9,0xf8,0x48,0x08,
        ];
        let mut w = [
            0xd4,0x27,0x11,0xae,
            0xe0,0xbf,0x98,0xf1,
            0xb8,0xb4,0x5d,0xe5,
            0x1e,0x41,0x52,0x30,
        ];
        inv_sub_bytes(&mut w);
        for i in 0..a.len() {
            assert_eq!(a[i], w[i]);
        }
    }
    #[test]
    fn test_shift_rows() {
        let mut w = [
            0xd4,0x27,0x11,0xae,
            0xe0,0xbf,0x98,0xf1,
            0xb8,0xb4,0x5d,0xe5,
            0x1e,0x41,0x52,0x30,
        ];
        let a = [
            0xd4,0xbf,0x5d,0x30,
            0xe0,0xb4,0x52,0xae,
            0xb8,0x41,0x11,0xf1,
            0x1e,0x27,0x98,0xe5,
        ];
        shift_rows(&mut w);
        arr_as_hex(&w);
        arr_as_hex(&a);
        for i in 0..a.len() {
            assert_eq!(a[i], w[i]);
        }
    }
    #[test]
    fn test_inv_shift_rows() {
        let a = [
            0xd4,0x27,0x11,0xae,
            0xe0,0xbf,0x98,0xf1,
            0xb8,0xb4,0x5d,0xe5,
            0x1e,0x41,0x52,0x30,
        ];
        let mut w = [
            0xd4,0xbf,0x5d,0x30,
            0xe0,0xb4,0x52,0xae,
            0xb8,0x41,0x11,0xf1,
            0x1e,0x27,0x98,0xe5,
        ];
        inv_shift_rows(&mut w);
        for i in 0..a.len() {
            assert_eq!(a[i], w[i]);
        }
    }
    #[test]
    fn test_mix_cols() {
        let mut w = [
            0xd4,0xbf,0x5d,0x30,
            0xe0,0xb4,0x52,0xae,
            0xb8,0x41,0x11,0xf1,
            0x1e,0x27,0x98,0xe5,
        ];
        let a = [
            0x04,0x66,0x81,0xe5,
            0xe0,0xcb,0x19,0x9a,
            0x48,0xf8,0xd3,0x7a,
            0x28,0x06,0x26,0x4c,
        ];
        mix_cols(&mut w);

        for i in 0..a.len() {
            assert_eq!(a[i], w[i]);
        }
    }
    #[test]
    fn test_inv_mix_cols() {
        let mut w = [
            0x04,0x66,0x81,0xe5,
            0xe0,0xcb,0x19,0x9a,
            0x48,0xf8,0xd3,0x7a,
            0x28,0x06,0x26,0x4c,
        ];
        let a = [
            0xd4,0xbf,0x5d,0x30,
            0xe0,0xb4,0x52,0xae,
            0xb8,0x41,0x11,0xf1,
            0x1e,0x27,0x98,0xe5,
        ];
        inv_mix_cols(&mut w);

        for i in 0..a.len() {
            assert_eq!(a[i], w[i]);
        }
    }
    #[test]
    fn test_encrypt_block() {
        let i = [
            0x32,0x43,0xf6,0xa8,
            0x88,0x5a,0x30,0x8d,
            0x31,0x31,0x98,0xa2,
            0xe0,0x37,0x07,0x34,
        ];
        let k = [
            0x2b,0x7e,0x15,0x16,
            0x28,0xae,0xd2,0xa6,
            0xab,0xf7,0x15,0x88,
            0x09,0xcf,0x4f,0x3c,
        ];
        let a = [
            0x39,0x25,0x84,0x1d,
            0x02,0xdc,0x09,0xfb,
            0xdc,0x11,0x85,0x97,
            0x19,0x6a,0x0b,0x32,
        ];
        let mut o = [0; 16];
        encrypt_block(&k, &i, &mut o);

        for i in 0..a.len() {
            assert_eq!(a[i],o[i]);
        }
    }
    #[test]
    fn test_decrypt_block() {
        let i = [
            0x39,0x25,0x84,0x1d,
            0x02,0xdc,0x09,0xfb,
            0xdc,0x11,0x85,0x97,
            0x19,0x6a,0x0b,0x32,
        ];
        let k = [
            0x2b,0x7e,0x15,0x16,
            0x28,0xae,0xd2,0xa6,
            0xab,0xf7,0x15,0x88,
            0x09,0xcf,0x4f,0x3c,
        ];
        let a = [
            0x32,0x43,0xf6,0xa8,
            0x88,0x5a,0x30,0x8d,
            0x31,0x31,0x98,0xa2,
            0xe0,0x37,0x07,0x34,
        ];
        let mut o = [0; 16];
        decrypt_block(&k, &i, &mut o);

        for i in 0..a.len() {
            assert_eq!(a[i],o[i]);
        }
    }
}

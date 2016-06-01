/* Compute discrete log modulo a prime p
 *
 * Let g be some element in Z_p
 * You are given h in Z_p such that h = g^x where 1 <= x <= 2^40
 *
 * brute force by finding an x satisfying h=g^x takes 2^40 multiplications
 * meet in the middle attack by solving
 * h/g^x_1 = (g^B)^x_0
 * where x = Bx_0 + x_1
 */
extern crate gmp;

use gmp::mpz::*;
use std::collections::HashMap;
use std::convert::From;

fn discrete_log(p: &Mpz, g: &Mpz, h: &Mpz) -> Option<Mpz> {
    let b =
    {
        let t:Mpz = From::<i64>::from(2);
        t.pow(20)
    };
    let mut left = HashMap::with_capacity(2u64.pow(20) as usize + 1);
    for x1u in 0u64..2u64.pow(20)+1 {
        let x1:Mpz = From::<u64>::from(x1u);
        let t0 = g.powm(&x1, &p);
        let t1 = t0.invert(p).unwrap();
        let t2 = (h * &t1).modulus(p);
        left.insert(t2, x1);
    }
    for x0u in 0u64..2u64.pow(20)+1 {
        let x0: Mpz = From::<u64>::from(x0u);
        let t = g.powm(&b, &p).powm(&x0, &p);
        match left.get(&t) {
            None => (),
            Some(x1) =>
            {
                let t0 = (&b * &x0).modulus(p);
                let t1 = (&t0 + x1).modulus(p);
                return Some(t1);
            },
        }
    }
    return None;
}

fn main() {
	let pstr =
		"134078079299425970995740249982058461274793658205923933\
		77723561443721764030073546976801874298166903427690031\
		858186486050853753882811946569946433649006084171";
    let gstr =
        "11717829880366207009516117596335367088558084999998952205\
         59997945906392949973658374667057217647146031292859482967\
         5428279466566527115212748467589894601965568";
    let hstr =
        "323947510405045044356526437872806578864909752095244\
         952783479245297198197614329255807385693795855318053\
         2878928001494706097394108577585732452307673444020333";
    let p = Mpz::from_str_radix(pstr, 10).unwrap();
    let g = Mpz::from_str_radix(gstr, 10).unwrap();
    let h = Mpz::from_str_radix(hstr, 10).unwrap();
    println!("{:?}", discrete_log(&p, &g, &h));
}


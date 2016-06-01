extern crate gmp;

use gmp::mpz::*;

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

fn sqrt_ceil(x: &Mpz) -> Mpz {
    let one = Mpz::one();
    let t = x.sqrt();
    if t.pow(2) < *x {
        return &t + &one;
    } else {
        return t;
    }
}

/* A - N.sqrt() < 1 */
fn factor1(n: &Mpz) -> (Mpz, Mpz) {
    let a = sqrt_ceil(n);
    let a2 = a.pow(2);
    let x = (&a2 - n).sqrt();
    let p = &a + &x;
    let q = &a - &x;
    return (p, q);
}

const NSTR: &'static str =
    "17976931348623159077293051907890247336179769789423065727343008115\
     77326758055056206869853794492129829595855013875371640157101398586\
     47833778606925583497541085196591615128057575940752635007475935288\
     71082364994994077189561705436114947486504671101510156394068052754\
     0071584560878577663743040086340742855278549092581";

fn challenge1() {
    let n = Mpz::from_str_radix(NSTR, 10).unwrap();
    let (p,q) = factor1(&n);
    let pq = &p * &q;
    println!("N: {:?}\npq: {:?}\np: {:?}\nq: {:?}", &n, &pq, &p, &q);
}

/* A - N.sqrt() < 2^20 */
fn factor2(n: &Mpz) -> Option<(Mpz, Mpz)> {
    let sqrt_n = sqrt_ceil(n);
    for i in 0..2u64.pow(20) {
        let a = i + &sqrt_n;
        let a2 = a.pow(2);
        let x = (&a2 - n).sqrt();
        let p = &a + &x;
        let q = &a - &x;
        if &p * &q == *n {
            return Some((p, q));
        }
    }
    return None;
}

const N2STR: &'static str =
    "6484558428080716696628242653467722787263437207069762630604390703787\
     9730861808111646271401527606141756919558732184025452065542490671989\
     2428844841839353281972988531310511738648965962582821502504990264452\
     1008852816733037111422964210278402893076574586452336833570778346897\
     15838646088239640236866252211790085787877";


fn challenge2() {
    let n = Mpz::from_str_radix(N2STR, 10).unwrap();
    let (p,q) = factor2(&n).unwrap();
    let pq = &p * &q;
    println!("N: {:?}\npq: {:?}\np: {:?}\nq: {:?}", &n, &pq, &p, &q);
}

/* |3p-2q|<N^(1/4) */
fn factor3(n: &Mpz) -> Option<(Mpz,Mpz)> {
    let six: Mpz = From::<i32>::from(6);
    let two: Mpz = From::<i32>::from(2);
    let three: Mpz = From::<i32>::from(3);
    let a = sqrt_ceil(&(&six * n));
    let ca = Mpz::one();
    let cb = -Mpz::one();
    let cc =
    {
        let t0 = a.pow(2);
        let t1 = &t0 - &a;
        let t2: Mpz = &six * n;
        -(&t1 - &t2)
    };
    let disc: Mpz =
    {
        let t0: Mpz = &cb * &cb;
        let four: Mpz = From::<i32>::from(4);
        let t1: Mpz = four * (&ca * &cc);
        (t0 - t1).sqrt()
    };
    let r0:Mpz = (cb.clone() - disc.clone()) / -(&two * &ca);
    let p = (a.clone() + r0.clone() - Mpz::one()) / three.clone();
    let q = (a.clone() - r0.clone()) / two.clone();
    if &p * &q == *n {
        return Some((p,q));
    }

    let r1 = (cb.clone() + disc.clone()) / -(&two * &ca);
    let p = (a.clone() + r1.clone() - Mpz::one()) / three.clone();
    let q = (a.clone() - r1.clone()) / two.clone();
    if &p * &q == *n {
        return Some((p,q));
    }
    return None;
}

const N3STR: &'static str =
    "72006226374735042527956443552558373833808445147399984182665305798191\
    63556901883377904234086641876639384851752649940178970835240791356868\
    77441155132015188279331812309091996246361896836573643119174094961348\
    52463970788523879939683923036467667022162701835329944324119217381272\
    9276147530748597302192751375739387929";

fn challenge3() {
    let n = Mpz::from_str_radix(N3STR, 10).unwrap();
    let (p,q) = factor3(&n).unwrap();
    let pq = &p * &q;
    println!("N: {:?}\npq: {:?}\np: {:?}\nq: {:?}", &n, &pq, &p, &q);
}


const CT4: &'static str =
    "22096451867410381776306561134883418017410069787892831071731839143676135600120538004282\
     32965047350942434394621975151225646583996794288946076454204058156474898801373486412045\
     23252293201764879166664029975091887299716905260832220677716000193292608700095799937240\
     77458967773697817571267229951148662959627934791540";

fn challenge4() {
    let n = Mpz::from_str_radix(NSTR, 10).unwrap();
    let (p,q) = factor1(&n);
    let e:Mpz = From::<i32>::from(65537);
    let ct = Mpz::from_str_radix(CT4, 10).unwrap();
    let phi_n = (&p - 1) * (&q - 1);
    let d = e.invert(&phi_n).unwrap();
    let pt = ct.powm(&d, &n);
    let pts = pt.to_str_radix(16).chars().collect::<Vec<char>>();
    let mut i = 0;
    while i < pts.len() {
        if pts[i] == '0' {
            if pts[i+1] == '0' {
                break;
            }
            i += 1;
        } else {
            i += 2;
        }
    }
    let msg = as_u8(&&pts[i+2..].iter().cloned().collect::<String>()).iter().map(|&b| b as char).collect::<String>(); //from_u8(&pts[i..]);
    println!("{}", &msg);
}
/*
impl fmt::LowerHex for Mpz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.to_str_radix(16);
        write!(f, "{}", s);
    }
}
*/


fn main() {
    challenge1();
    println!("");
    challenge2();
    println!("");
    challenge3();
    println!("");
    challenge4();
}

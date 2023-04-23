use fraction::BigFraction;
use num::{bigint::ToBigInt, BigInt, BigRational, FromPrimitive, ToPrimitive};
fn compute_m(m: BigInt, q: u128) -> BigInt {
    let numerator = (12 * q + 2) * (12 * q + 6) * (12 * q + 10);
    let denominator = (q + 1).pow(3);
    return m * (numerator / denominator).to_bigint().unwrap();
}
fn compute_l(l: BigInt) -> BigInt {
    return l + BigInt::from_i64(545140134).unwrap();
}
fn compute_x(x: BigInt) -> BigInt {
    return x * (-262537412640768000 as i128).to_bigint().unwrap();
}
fn main() {
    let c = 426880 as f64 * (10005 as f64).sqrt();
    let n = 128;
    let mut total: BigRational = BigRational::from_integer(0.to_bigint().unwrap());
    let mut m: BigInt = BigInt::from(1);
    let mut x: BigInt = BigInt::from(1);
    let mut l: BigInt = BigInt::from(13591409);
    for q in 0..n {
        total += BigRational::new(m.clone() * l.clone(), x.clone());
        m = compute_m(m, q);
        l = compute_l(l);
        x = compute_x(x);
    }
    total += BigRational::new(m * l, x);
    let value = total.recip();
    let pi = BigRational::from_f64(c).unwrap() * value;
    // let numer = BigFraction::from(pi.numer().to_bigint().unwrap());
    // let denom = BigFraction::from(pi.denom().to_bigint().unwrap());
    // let thing = numer / denom;
    println!("{}", pi.to_f64().unwrap())
}

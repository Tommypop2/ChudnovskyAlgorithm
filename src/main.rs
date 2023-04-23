fn factorial(n: f64) -> f64 {
    if n == 0.0 {
        return 1.0;
    }
    if n == 1.0 {
        return 1.0;
    }
    let mut total: f64 = 2.0;
    for i in 3..(n as u64 + 1) {
        total *= i as f64;
    }
    return total;
}
fn compute_m(q: f64) -> f64 {
    let numerator = factorial(6.0 * q);
    let denominator = factorial(3.0 * q) * factorial(q).powf(3.0);
    return numerator / denominator;
}
fn compute_l(q: f64) -> f64 {
    return 545140134.0 * q + 13591409.0;
}
fn compute_x(q: f64) -> i128 {
    return (-262537412640768000 as i128).pow(q as u32);
}
fn main() {
    let c = 426880 as f64 * (10005 as f64).sqrt();
    let n = 3;
    let mut total: f64 = 0.0;
    for q in 0..n {
        let m = compute_m(q.into());
        let l = compute_l(q.into());
        let x = compute_x(q.into());
        total += (m * l) / x as f64;
    }
    let value = 1.0 / total;
    let pi = c * value;
    println!("{}", pi)
}

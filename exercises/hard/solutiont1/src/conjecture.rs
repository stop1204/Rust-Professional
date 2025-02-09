#[inline]
fn can_be_expressed(n: u64) -> bool {
    for p in (2..n).filter(|&x| is_prime(x)) {
        // p + remainder = n
        // remainder =  2 * j^2
        let remainder = n - p;
        // remainder 必定是偶数
        if remainder % 2 == 0 {
            let j = (remainder / 2) as f64;
            // 完全平方数
            if j.sqrt().fract() == 0.0 {
                return true;
            }
        }
    }
    false
}


#[inline]
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}


#[inline]
pub fn goldbach_conjecture() -> String {
    (9..10000000)
        .step_by(2)
        .filter(|&i| !is_prime(i) && !can_be_expressed(i))
        .take(2)
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

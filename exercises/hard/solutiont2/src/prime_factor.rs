#[inline]
pub fn find_max_prime_factor(mut n: u128) -> u128 {
    let nn = n.clone();
    if n <= 1 {
        return n;
    }

    let mut max_factor = 1;
    while n % 2 == 0 {
        max_factor = 2;
        n /= 2;
    }
    if is_prime(n) {
        return n;
    }


    let sqrt_n = ((n as f64).sqrt() as u128) * 1000;

    let mut factor = 3;
    while factor * factor <= sqrt_n {
        if n % factor == 0 {
            max_factor = factor;
            n /= factor;
        } else {
            factor += 2;
        }
    }


    if n > max_factor {
        n
    } else {
        max_factor
    }
}


#[inline]
fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u128 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
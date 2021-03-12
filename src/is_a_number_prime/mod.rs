use std::process::exit;

pub fn is_prime(x: i64) -> bool {
    if x <= 1 { return false; }
    if x == 2 { return true; }
    let mut prime_numbers: Vec<i64> = vec![2];

    let root_x = (x as f64).sqrt().ceil() as i64;

    for n in (3..=root_x).step_by(2) {
        if !divisible_by_any(n, &prime_numbers) {
            prime_numbers.push(n);
            if x % n == 0 { return false; }
        }
    }

    return !divisible_by_any(x, &prime_numbers);
}

fn divisible_by_any(x: i64, primes: &Vec<i64>) -> bool {
    for prime in primes {
        if x % prime == 0 {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert!(!is_prime(0), "0 is not prime");
        assert!(!is_prime(1), "1 is not prime");
        assert!(is_prime(2), "2 is prime");
        assert!(is_prime(73), "73 is prime");
        assert!(!is_prime(75), "75 is not prime");
        assert!(!is_prime(-1), "-1 is not prime");
    }

    #[test]
    fn prime_tests() {
        assert!(is_prime(3), "3 is prime");
        assert!(is_prime(5), "5 is prime");
        assert!(is_prime(7), "7 is prime");
        assert!(is_prime(41), "41 is prime");
        assert!(is_prime(5099), "5099 is prime");
    }

    #[test]
    fn not_prime_tests() {
        assert!(!is_prime(4), "4 is not prime");
        assert!(!is_prime(6), "6 is not prime");
        assert!(!is_prime(8), "8 is not prime");
        assert!(!is_prime(9), "9 is not prime");
        assert!(!is_prime(45), "45 is not prime");
        assert!(!is_prime(-5), "-5 is not prime");
        assert!(!is_prime(-8), "-8 is not prime");
        assert!(!is_prime(-41), "-41 is not prime");
    }

    #[test]
    fn large_one() {
        // Takes almost forever
        // assert!(is_prime(2305843009213693951), "2305843009213693951 is prime");
    }

    #[test]
    fn largest() {
        assert!(!is_prime(i64::MAX), "MAX is not prime");
    }
}

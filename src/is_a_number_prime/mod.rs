pub fn is_prime(n: i64) -> bool {
    match n {
        2 | 3 => true,
        x if x <= 1 => false,
        x if x % 2 == 0 || x % 3 == 0 => false,
        _ => {
            let sqrt_n = ((n as f64).sqrt() as i64) + 1;
            !(6..=sqrt_n)
                .step_by(6)
                .any(|i| n % (i - 1) == 0 || n % (i + 1) == 0)
        }
    }
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
        assert!(!is_prime(1494466603), "1494466603 is not prime");
        assert!(!is_prime(1021633369), "1021633369 is not prime");
    }

    #[test]
    fn large_one() {
        // Takes almost forever
        assert!(is_prime(2305843009213693951), "2305843009213693951 is prime");
    }

    #[test]
    #[ignore]
    fn largest() {
        assert!(!is_prime(i64::MAX), "MAX is not prime");
    }
}

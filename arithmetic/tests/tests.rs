#![feature(slice_patterns)]

extern crate arithmetic;

#[test]
fn test_is_prime() {
    // 31 Determine whether a given integer number is prime.
    assert_eq!(arithmetic::is_prime(7), true);
    assert_eq!(arithmetic::is_prime(6), false);
    assert_eq!(arithmetic::is_prime(17), true);
    assert_eq!(arithmetic::is_prime(97), true);
    assert_eq!(arithmetic::is_prime(60), false);
}

#[test]
fn test_gcd() {
    // 32 Determine the greatest common divisor of two positive integer numbers. Use Euclid's algorithm.
    assert_eq!(arithmetic::gcd(36, 63), 9);
    assert_eq!(arithmetic::gcd(-3, -6), 3);
    assert_eq!(arithmetic::gcd(-3, 6), 3);
}

#[test]
fn test_coprime() {
    // 33 Determine whether two positive integer numbers are coprime.
    // Two numbers are coprime if their greatest common divisor equals 1.
    assert_eq!(arithmetic::coprime(35, 64), true);
}

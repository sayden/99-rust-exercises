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
    assert_eq!(arithmetic::is_prime(2), true);
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

#[test]
fn test_totient() {
    // 34 Calculate Euler's totient function phi(m).
    assert_eq!(arithmetic::totient(10), 4);
    assert_eq!(arithmetic::totient(45), 24);
}

#[test]
fn test_prime_factors() {
    // 35 Determine the prime factors of a given positive integer. Construct a flat list containing the prime factors in ascending order.
    assert_eq!(arithmetic::prime_factors(315), vec![3, 3, 5, 7]);
    assert_eq!(arithmetic::prime_factors(20), vec![2, 2, 5]);
    assert_eq!(arithmetic::prime_factors(21), vec![3, 7]);
}

#[test]
fn test_prime_factors_multi() {
    // 36 Determine the prime factors of a given positive integer. Construct a list containing the prime factors and their multiplicity.
    assert_eq!(arithmetic::prime_factors_multi(315),
               vec![(2, 3), (1, 5), (1, 7)]);
}

#[test]
fn test_totient_improved() {
    // 37 Calculate Euler's totient function phi(m) (improved)
    assert_eq!(arithmetic::totient_improved(10), 4);
    assert_eq!(arithmetic::totient_improved(45), 24);
}

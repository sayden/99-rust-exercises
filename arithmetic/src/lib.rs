#![feature(slice_patterns)]
#![feature(default_type_parameter_fallback)]
#![feature(iter_arith)]

extern crate lists;

// 31 Determine whether a given integer number is prime.
pub fn is_prime(n: u32) -> bool {
    if n > 1 {
        for i in 2..n {
            if i * i <= n {
                match n.checked_rem(i) {
                    None => return false,
                    Some(e) => {
                        match e {
                            0 => return false,
                            _ => (),
                        }
                    }
                }
            }
        }

        return true;
    } else {
        return true;
    }
}

// 32 Determine the greatest common divisor of two positive integer numbers. Use Euclid's algorithm.
pub fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x.abs()
    } else {
        gcd(y, (x.checked_rem(y).unwrap()))
    }
}


// 33 Determine whether two positive integer numbers are coprime.
// Two numbers are coprime if their greatest common divisor equals 1.
pub fn coprime(x: i32, y: i32) -> bool {
    gcd(x, y) == 1
}

// 34 Calculate Euler's totient function phi(m).
pub fn totient(n: i32) -> i32 {
    match n {
        1 => 1,
        a => (1..a).into_iter().filter(|b| coprime(*b, n)).collect::<Vec<i32>>().len() as i32,
    }
}

// 35 Determine the prime factors of a given positive integer. Construct a flat list containing the prime factors in ascending order.
pub fn prime_factors(n: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    for i in 2..n {
        if is_prime(i) {
            primes.push(i);
        }
    }

    let mut _n = n;

    let mut res: Vec<u32> = Vec::new();

    for i in primes {
        while _n % i == 0 {
            res.push(i);
            _n = _n / i;
        }
    }

    res
}

// 36 Determine the prime factors of a given positive integer. Construct a list containing the prime factors and their multiplicity.
pub fn prime_factors_multi(n: u32) -> Vec<(usize, u32)> {
    let prime_f = prime_factors(n);

    lists::encode(prime_f)
}

// 37 Calculate Euler's totient function phi(m) (improved)
pub fn totient_improved(n: u32) -> u32 {
    let factors = prime_factors_multi(n);

    let mut res = Vec::new();
    for (q, i) in factors {
        res.push((i - 1) * i.pow((q as u32) - 1));
    }

    res.into_iter().product()
}

// 39 A list of prime numbers.
pub fn primes_r(x: u32, y: u32) -> Vec<u32> {
    let mut res = Vec::new();
    for i in x..y {
        if is_prime(i) {
            res.push(i);
        }
    }

    res
}

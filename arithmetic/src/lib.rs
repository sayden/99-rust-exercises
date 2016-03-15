#![feature(slice_patterns)]
#![feature(default_type_parameter_fallback)]

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

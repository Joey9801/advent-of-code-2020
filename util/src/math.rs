use crate::integer::Integer;

pub fn gcd<T: Integer>(a: T, b: T) -> T {
    if b == T::zero() { a } else { gcd(b, a % b) }
}

/// Finds the greatest common divisor of a and b, and the bezout coefficients of a and b
pub fn egcd<T: Integer>(a: T, b: T) -> (T, T, T) {
    if a == T::zero() {
        (b, T::zero(), T::one())
    }
    else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}


/// Computes an integer `x` s.t. `(a * x) % m == 1`
pub fn modinverse<T: Integer>(a: T, m: T) -> Option<T> {
    let (g, x, _) = egcd(a, m);
    if g != T::one() {
        None
    }
    else {
        Some((x % m + m) % m)
    }
}

pub fn lcm<T: Integer>(a: T, b: T) -> T {
    a * b  / gcd(a, b)
}

pub fn lcm3<T: Integer>(a: T, b: T, c: T) -> T {
    lcm(a, lcm(b, c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_inverse() {
        // 3 * 77 = 231 = 46 * 5 + 1 = 1 (mod 5)
        assert_eq!(modinverse(77, 5), Some(3));
        
        // 6 * 55 = 330 = 47 * 7 + 1 = 1 (mod 7)
        assert_eq!(modinverse(55, 7), Some(6));
        
        // 6 * 35 == 210 = 19 * 11 + 1 = 1 (mod 11)
        assert_eq!(modinverse(35, 11), Some(6));

        // 32 = 8 * 4
        assert_eq!(modinverse(4, 32), None);
    }
}
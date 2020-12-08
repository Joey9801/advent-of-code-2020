use crate::integer::Integer;

pub fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

pub fn gcd<T: Integer>(a: T, b: T) -> T {
    if b == T::zero() { a } else { gcd(b, a % b) }
}

pub fn lcm<T: Integer>(a: T, b: T) -> T {
    a * b  / gcd(a, b)
}

pub fn lcm3<T: Integer>(a: T, b: T, c: T) -> T {
    lcm(a, lcm(b, c))
}
// https://www.hackerrank.com/challenges/between-two-sets/problem

/// Функція для знаходження кількості чисел між двома множинами.
/// Використовує snake_case для відповідності стандартам Rust та відсутності warnings.
pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    if a.is_empty() || b.is_empty() {
        return 0;
    }

    let mut l = a[0];
    for &val in a.iter().skip(1) {
        l = lcm(l, val);
    }

    let mut g = b[0];
    for &val in b.iter().skip(1) {
        g = gcd(g, val);
    }

    if l > g {
        return 0;
    }

    let mut count = 0;
    let mut multiple = l;
    while multiple <= g {
        if g % multiple == 0 {
            count += 1;
        }
        multiple += l;
    }

    count
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn lcm(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        return 0;
    }
    // Запобігання переповненню: (a / gcd(a, b)) * b
    (a / gcd(a, b)) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }

    #[test]
    fn test_case_1() {
        let a = vec![2, 6];
        let b = vec![24, 36];
        assert_eq!(get_total_x(&a, &b), 2);
    }

    #[test]
    fn test_empty() {
        assert_eq!(get_total_x(&[], &[1]), 0);
    }
}

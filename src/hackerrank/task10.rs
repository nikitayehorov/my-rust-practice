// https://www.hackerrank.com/challenges/sock-merchant/problem
use std::collections::HashMap;

pub fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    let mut counts = HashMap::new();
    for &sock in ar {
        *counts.entry(sock).or_insert(0) += 1;
    }

    let mut pairs = 0;
    for count in counts.values() {
        pairs += count / 2;
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let n = 9;
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(n, &ar), 3);
    }

    #[test]
    fn test_case_1() {
        let n = 10;
        let ar = vec![1, 1, 3, 1, 2, 1, 3, 3, 3, 3];
        assert_eq!(sock_merchant(n, &ar), 4);
    }
}

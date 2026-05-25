// https://www.hackerrank.com/challenges/divisible-sum-pairs/problem

/// Функція для підрахунку пар (i, j), де i < j та (ar[i] + ar[j]) ділиться на k.
pub fn divisible_sum_pairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    let n_usize = n as usize;

    for i in 0..n_usize {
        for j in (i + 1)..n_usize {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let k = 3;
        let ar = vec![1, 3, 2, 6, 1, 2];
        assert_eq!(divisible_sum_pairs(6, k, &ar), 5);
    }

    #[test]
    fn test_case_1() {
        let k = 5;
        let ar = vec![1, 2, 3, 4, 5, 6];
        // Pairs: (1,4), (2,3), (4,6) -> 3 pairs
        assert_eq!(divisible_sum_pairs(6, k, &ar), 3);
    }
}

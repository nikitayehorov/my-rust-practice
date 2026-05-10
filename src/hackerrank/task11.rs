// https://www.hackerrank.com/challenges/diagonal-difference/problem

/// Функція для обчислення абсолютної різниці між сумами діагоналей квадратної матриці.
pub fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_sum = 0;
    let mut secondary_sum = 0;

    for i in 0..n {
        primary_sum += arr[i][i];
        secondary_sum += arr[i][n - 1 - i];
    }

    (primary_sum - secondary_sum).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let arr = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![9, 8, 9],
        ];
        // 1+5+9 = 15
        // 3+5+9 = 17
        // |15 - 17| = 2
        assert_eq!(diagonal_difference(&arr), 2);
    }

    #[test]
    fn test_case_1() {
        let arr = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];
        // 11+5-12 = 4
        // 4+5+10 = 19
        // |4 - 19| = 15
        assert_eq!(diagonal_difference(&arr), 15);
    }
}

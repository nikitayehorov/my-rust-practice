// https://www.hackerrank.com/challenges/birthday-cake-candles/problem

/// Функція для підрахунку кількості найвищих свічок.
pub fn birthday_cake_candles(candles: &[i32]) -> i32 {
    if candles.is_empty() {
        return 0;
    }

    let mut max_val = candles[0];
    let mut count = 0;

    for &h in candles {
        if h > max_val {
            max_val = h;
            count = 1;
        } else if h == max_val {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let candles = vec![4, 4, 1, 3];
        assert_eq!(birthday_cake_candles(&candles), 2);
    }

    #[test]
    fn test_case_1() {
        let candles = vec![3, 2, 1, 3];
        assert_eq!(birthday_cake_candles(&candles), 2);
    }

    #[test]
    fn test_single() {
        let candles = vec![5];
        assert_eq!(birthday_cake_candles(&candles), 1);
    }
}

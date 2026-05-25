// https://www.hackerrank.com/challenges/drawing-book/problem

/// Функція для знаходження мінімальної кількості перегортань сторінок.
pub fn page_count(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = (n / 2) - (p / 2);
    std::cmp::min(from_front, from_back)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(page_count(6, 2), 1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(page_count(5, 4), 0);
    }

    #[test]
    fn test_case_1() {
        assert_eq!(page_count(6, 5), 1);
    }
}

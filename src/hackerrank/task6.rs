#[allow(dead_code)]
pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    // Якщо перший кенгуру стартує позаду, але він повільніший або такий самий, він ніколи не наздожене
    if v1 <= v2 {
        return "NO".to_string();
    }
    
    // Перевіряємо, чи відстань між ними скорочується до 0 за цілу кількість стрибків
    // Формула: (x2 - x1) % (v1 - v2) == 0
    if (x2 - x1) % (v1 - v2) == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo_will_meet() {
        // Sample Input 0: 0 3 4 2 -> YES
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_kangaroo_will_not_meet() {
        // Sample Input 1: 0 2 5 3 -> NO
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_kangaroo_same_speed_different_start() {
        assert_eq!(kangaroo(0, 2, 5, 2), "NO");
    }
}

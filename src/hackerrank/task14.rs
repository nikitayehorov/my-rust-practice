// https://www.hackerrank.com/challenges/bon-appetit/problem

/// Функція для обчислення рахунку для Анни та перевірки правильності розрахунку Браяна.
/// Якщо розрахунок правильний, повертає None (виведеться "Bon Appetit").
/// Інакше повертає суму повернення Браяна Анні.
pub fn bon_appetit(bill: &[i32], k: usize, b: i32) -> Option<i32> {
    let total_sum: i32 = bill.iter().sum();
    let actual_share = (total_sum - bill[k]) / 2;
    if b == actual_share {
        None
    } else {
        Some(b - actual_share)
    }
}

/// Допоміжна функція для виведення результату у стандартний потік, відповідно до умов HackerRank.
pub fn print_bon_appetit(bill: &[i32], k: usize, b: i32) {
    match bon_appetit(bill, k, b) {
        None => println!("Bon Appetit"),
        Some(refund) => println!("{}", refund),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_bon_appetit() {
        let bill = vec![3, 10, 2, 9];
        let k = 1;
        let b = 12; // sum of indices [0, 2, 3] = 3 + 2 + 9 = 14. 14 / 2 = 7. Refund 12 - 7 = 5.
        assert_eq!(bon_appetit(&bill, k, b), Some(5));
    }

    #[test]
    fn test_example_bon_appetit_correct() {
        let bill = vec![3, 10, 2, 9];
        let k = 1;
        let b = 7; // sum = 14 / 2 = 7. Correct!
        assert_eq!(bon_appetit(&bill, k, b), None);
    }

    #[test]
    fn test_print_output() {
        // Just call it to prevent dead_code compiler warnings
        print_bon_appetit(&[3, 10, 2, 9], 1, 12);
    }
}

#[allow(dead_code)]
pub fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) -> (i32, i32) {
    // Рахуємо яблука: позиція дерева + відстань, потім фільтруємо за діапазоном будинку
    let apple_count = apples.iter()
        .map(|&d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    // Рахуємо апельсини: позиція дерева + відстань, фільтруємо
    let orange_count = oranges.iter()
        .map(|&d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    println!("{}", apple_count);
    println!("{}", orange_count);
    
    (apple_count, orange_count) // Повертаємо для тесту
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hackerrank_sample() {
        // Тест із прикладу: будинок 7-11, дерева 5 та 15
        // Яблука: [-2, 2, 1] -> позиції: [3, 7, 6]. Тільки 7 в діапазоні.
        // Апельсини: [5, -6] -> позиції: [20, 9]. Тільки 9 в діапазоні.
        let (app, org) = count_apples_and_oranges(7, 11, 5, 15, &[-2, 2, 1], &[5, -6]);
        assert_eq!(app, 1);
        assert_eq!(org, 1);
    }
}

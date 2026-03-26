#[allow(dead_code)]
pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        // Правило 2: Якщо оцінка менша за 38 - не округляємо
        if grade < 38 {
            grade
        } else {
            // Шукаємо різницю до наступного числа, кратного 5
            let rem = grade % 5;
            let diff = 5 - rem;

            // Правило 1: Якщо різниця менша за 3 - округляємо вгору
            if diff < 3 {
                grade + diff
            } else {
                grade
            }
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hackerrank_sample() {
        // Тест на основі Sample Input з твоєї умови
        let input = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(&input), expected);
    }

    #[test]
    fn test_edge_cases() {
        // 84 -> 85 (різниця 1 < 3)
        // 57 -> 57 (різниця 3, не округляємо)
        let input = vec![84, 29, 57];
        let expected = vec![85, 29, 57];
        assert_eq!(grading_students(&input), expected);
    }
}

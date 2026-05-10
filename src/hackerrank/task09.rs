// https://www.hackerrank.com/challenges/migratory-birds/problem

pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6];
    for &bird in arr {
        if bird >= 1 && bird <= 5 {
            counts[bird as usize] += 1;
        }
    }

    let mut max_count = 0;
    let mut result_id = 0;

    for i in 1..=5 {
        if counts[i] > max_count {
            max_count = counts[i];
            result_id = i as i32;
        }
    }

    result_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_ties() {
        let arr = vec![1, 2, 2, 4, 4, 4, 4, 5];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_multiple_ties() {
        let arr = vec![1, 1, 2, 2, 3];
        assert_eq!(migratory_birds(&arr), 1);
    }
}

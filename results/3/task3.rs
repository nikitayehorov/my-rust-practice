#[allow(dead_code)]
/// Функція staircase згідно з вимогами HackerRank
pub fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = n - i;
        let hashes = i;
        println!(
            "{}{}",
            " ".repeat(spaces as usize),
            "#".repeat(hashes as usize)
        );
    }
}

/// Функція для тестів (повертає рядок, щоб ми могли його перевірити)
pub fn get_staircase_string(n: i32) -> String {
    let mut out = String::new();
    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        out.push_str(&format!("{}{}", spaces, hashes));
        if i < n { out.push('\n'); }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_output() {
        let result = get_staircase_string(4);
        let expected = "   #\n  ##\n ###\n####";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_staircase_one() {
        let result = get_staircase_string(1);
        assert_eq!(result, "#");
    }
}

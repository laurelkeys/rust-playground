use anyhow::{Context, Result};

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for (index, line) in content.lines().enumerate() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)
                .with_context(|| format!("failed to read line #{}", index + 1))?;
        }
    }

    Ok(())
}

//
// Test functions.
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_validity() {
        let mut result = Vec::new();
        assert!(find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result).is_ok());
        assert_eq!(result, b"lorem ipsum\n");
    }
}
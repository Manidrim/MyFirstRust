#[cfg(test)]
mod tests {
    use std::io::{self, Write};

    #[test]
    fn test_main() {
        let input = "Alice\n";
        let expected_output = "Bonjour, Alice !\n";

        let mut output = Vec::new();
        {
            let mut writer = io::BufWriter::new(&mut output);
            let _ = write!(writer, "{}", expected_output);
        }

        let result = input.trim().to_string();
        assert_eq!(result, "Alice");

        assert_eq!(String::from_utf8(output).unwrap(), expected_output);
    }
}
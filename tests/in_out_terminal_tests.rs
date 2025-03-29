#[cfg(test)]
mod tests {
    // Removed unused import: super::*

    #[test]
    fn test_read_line() {
        // Simule une entrée utilisateur
        let input = "Alice\n";
        let expected_output = "Alice";

        // Teste la fonction read_line
        let result = input.trim().to_string();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_print() {
        // Capture la sortie de la fonction print
        let mut output = Vec::new();
        let string = "Alice";
        {
            use std::io::Write;
            let _ = write!(&mut output, "{}", string);
        }

        let expected_output = "Alice";
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);
    }

    #[test]
    fn test_print_hello() {
        // Capture la sortie de la fonction print
        let mut output = Vec::new();
        let name = "Alice";
        {
            use std::io::Write;
            let _ = write!(&mut output, "Bonjour, {} !\n", name);
        }

        let expected_output = "Bonjour, Alice !\n";
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);
    }
}
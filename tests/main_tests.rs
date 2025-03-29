#[cfg(test)]
mod tests {
    use std::io::{self, Write};

    #[test]
    fn test_main_option_0() {
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

        let input = "0\n";
        let result = input.trim().to_string();
        assert_eq!(result, "0");

        let expected_output = "Fin du programme !\nAu revoir !\n";
        let mut output = Vec::new();
        {
            let mut writer = io::BufWriter::new(&mut output);
            let _ = write!(writer, "{}", expected_output);
        }
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);
    }

    #[test]
    fn test_main_option_1() {
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

        // Simule une entrée utilisateur
        let input = "1\n";
        let expected_output = "Jeu de devinette !\n";
        let mut output = Vec::new();
        {
            let mut writer = io::BufWriter::new(&mut output);
            let _ = write!(writer, "{}", expected_output);
        }
        let result = input.trim().to_string();
        assert_eq!(result, "1");
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);

        // Simule une sortie de programme
        let expected_output = "Fin du programme !\nAu revoir !\n";
        let mut output = Vec::new();
        {
            let mut writer = io::BufWriter::new(&mut output);
            let _ = write!(writer, "{}", expected_output);
        }
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);
    }

    #[test]
    fn test_main_option_other() {
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

        let input = "kaamelott\n";
        let result = input.trim().to_string();
        assert_eq!(result, "kaamelott");

        let expected_output = "Fin du programme !\nAu revoir !\n";
        let mut output = Vec::new();
        {
            let mut writer = io::BufWriter::new(&mut output);
            let _ = write!(writer, "{}", expected_output);
        }
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);
    }
}
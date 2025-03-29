use std::io;

pub fn print_hello(nom: &str) {
    print(&format!("Bonjour, {} !", nom));
}

pub fn print(string: &str) {
    println!("{}", string);
}

pub fn read_line() -> String {
    let mut from_keyboard = String::new();
    io::stdin()
        .read_line(&mut from_keyboard)
        .expect("Échec de la lecture de l'entrée");
    return from_keyboard.trim().to_string();
}
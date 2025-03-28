use std::io;

fn main() {
    println!("Quel est votre nom ?");


    let nom = read_line();

    print(nom.as_str());
}

fn read_line() -> String {
    let mut from_keyboard = String::new();
    io::stdin()
        .read_line(&mut from_keyboard)
        .expect("Échec de la lecture de l'entrée");
    return from_keyboard.trim().to_string();
}

fn print(nom: &str) {
    println!("Bonjour, {} !", nom);
}


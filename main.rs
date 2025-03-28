use std::io; // Importe la bibliothèque standard pour les entrées/sorties.

fn main() {
    println!("Quel est votre nom ?");

    let mut nom = String::new();

    io::stdin()
        .read_line(&mut nom)
        .expect("Échec de la lecture de l'entrée");

    let nom = nom.trim();

    print(nom);
}

fn print(nom: &str) {
    println!("Bonjour, {} !", nom);
}
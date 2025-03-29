mod in_out_terminal;

use in_out_terminal::print;
use in_out_terminal::print_hello;
use in_out_terminal::read_line;

fn main() {
    print("Quel est votre nom ?");


    let nom: String = read_line();

    print_hello(nom.as_str());

    print("Que voulez-vous faire ?");
    print("0. Quitter");
    print("1. Jouer à trouver un nombre");

    let choix: String = read_line();
    if choix == "1" {
        print("Jeu de devinette !");
    }
    print("Fin du programme !");
    print("Au revoir !");
}
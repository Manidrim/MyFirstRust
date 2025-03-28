mod in_out_terminal;

use in_out_terminal::print;
use in_out_terminal::read_line;

fn main() {
    println!("Quel est votre nom ?");


    let nom: String = read_line();

    print(nom.as_str());
}



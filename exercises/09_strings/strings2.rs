// TODO: Corrige l'erreur de compilation dans la fonction `main` sans changer cette fonction.
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
   let word = String::from("green"); // Ne change pas cette ligne.

   if is_a_color_word(word) {
       println!("C'est une couleur que je connais !");
   } else {
       println!("Ce n'est pas une couleur que je connais.");
   }
}

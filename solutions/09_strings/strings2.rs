fn is_a_color_word(attempt: &str) -> bool {
   attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
   let word = String::from("green");

   if is_a_color_word(&word) {
       //             ^ ajouté pour avoir `&String` qui est automatiquement
       //               converti en `&str` par le compilateur.
       println!("C'est une couleur que je connais !");
   } else {
       println!("Ce n'est pas une couleur que je connais.");
   }
}

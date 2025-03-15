// Characters (`char`)

fn main() {
   // Note les guillemets _simples_, ils sont différents des guillemets doubles
   // que tu as vus jusqu'à présent.
   let my_first_initial = 'C';
   if my_first_initial.is_alphabetic() {
       println!("Alphabétique !");
   } else if my_first_initial.is_numeric() {
       println!("Numérique !");
   } else {
       println!("Ni alphabétique ni numérique !");
   }

   // TODO: De façon analogue à l'exemple précédent, déclare une variable appelée `your_character`
   // ci-dessous avec ton caractère préféré.
   // Essaie une lettre, essaie un chiffre (entre guillemets simples), essaie un caractère spécial, essaie un caractère
   // d'une langue différente de la tienne, essaie un emoji 😉
   // let your_character = '';

   if your_character.is_alphabetic() {
       println!("Alphabétique !");
   } else if your_character.is_numeric() {
       println!("Numérique !");
   } else {
       println!("Ni alphabétique ni numérique !");
   }
}

// TODO: Corrige l'erreur de compilation sur cette fonction.
fn picky_eater(food: &str) -> &str {
   if food == "fraise" {
       "Miam !"
   } else {
       "Je suppose que je peux manger ça."
   }
}

fn main() {
   // Tu peux faire des tests ici si tu le souhaites.
}

// TODO: Lis les tests pour comprendre le comportement souhaité.
// Fais passer tous les tests sans les modifier.
#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn yummy_food() {
       // Cela signifie que l'appel de `picky_eater` avec l'argument "fraise" devrait retourner "Miam !".
       assert_eq!(picky_eater("fraise"), "Miam !");
   }

   #[test]
   fn neutral_food() {
       assert_eq!(picky_eater("patate"), "Je suppose que je peux manger ça.");
   }

   #[test]
   fn default_disliked_food() {
       assert_eq!(picky_eater("brocoli"), "Non merci !");        
       assert_eq!(picky_eater("bonbons"), "Non merci !");
       assert_eq!(picky_eater("n'importe quoi"), "Non merci !");
   }
}

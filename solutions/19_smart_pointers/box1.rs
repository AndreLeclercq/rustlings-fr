// À la compilation, Rust doit connaître l'espace qu'un type occupe. Cela
// devient problématique pour les types récursifs, où une valeur peut avoir comme partie
// d'elle-même une autre valeur du même type. Pour contourner ce problème, nous pouvons utiliser un
// `Box` - un smart pointer (pointeur intelligent) utilisé pour stocker des données sur le heap (tas), qui nous permet également
// d'encapsuler un type récursif.
//
// Le type récursif que nous implémentons dans cet exercice est la "cons list" (liste chaînée),
// une structure de données fréquemment trouvée dans les langages de programmation fonctionnelle. Chaque
// élément dans une cons list contient deux éléments : La valeur de l'élément actuel et
// l'élément suivant. Le dernier élément est une valeur appelée `Nil`.

#[derive(PartialEq, Debug)]
enum List {
   Cons(i32, Box<List>),
   Nil,
}

fn create_empty_list() -> List {
   List::Nil
}

fn create_non_empty_list() -> List {
   List::Cons(42, Box::new(List::Nil))
}

fn main() {
   println!("Voici une cons list vide: {:?}", create_empty_list());
   println!(
       "Voici une cons list non vide: {:?}",
       create_non_empty_list(),
   );
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_create_empty_list() {
       assert_eq!(create_empty_list(), List::Nil);
   }

   #[test]
   fn test_create_non_empty_list() {
       assert_ne!(create_empty_list(), create_non_empty_list());
   }
}

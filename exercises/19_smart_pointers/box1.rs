// Lors de la compilation, Rust doit connaître l'espace occupé par un type. Cela
// devient problématique pour les types récursifs, où une valeur peut contenir
// une autre valeur du même type. Pour contourner ce problème, on peut utiliser
// un `Box` - un pointeur intelligent utilisé pour stocker des données sur le tas (heap),
// qui nous permet également d'envelopper un type récursif.
//
// Le type récursif qu'on implémente dans cet exercice est la "liste cons" (cons list),
// une structure de données fréquemment utilisée dans les langages de programmation 
// fonctionnelle. Chaque élément d'une liste cons contient deux éléments : 
// la valeur de l'élément courant et l'élément suivant. Le dernier élément 
// est une valeur appelée `Nil`.

// TODO: Utilise un `Box` dans la définition de l'enum pour faire compiler le code.
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

// TODO: Crée une liste cons vide.
fn create_empty_list() -> List {
    todo!()
}

// TODO: Crée une liste cons non vide.
fn create_non_empty_list() -> List {
    todo!()
}

fn main() {
    println!("Voici une liste cons vide : {:?}", create_empty_list());
    println!(
        "Voici une liste cons non vide : {:?}",
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

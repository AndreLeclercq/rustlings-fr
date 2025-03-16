// Cet exercice explore le pointeur intelligent `Cow` (Clone-On-Write). Il peut
// enfermer et fournir un accès immuable à des données empruntées et cloner les données
// de manière paresseuse quand une mutation ou une possession est requise. Le type
// est conçu pour travailler avec des données empruntées génériques via le trait `Borrow`.

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // Clone dans un vecteur si pas déjà possédé.
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // Un clone se produit car `input` doit être muté.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // Aucun clone ne se produit car `input` n'a pas besoin d'être muté.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // TODO: Remplace `todo!()` par `Cow::Owned(_)` ou `Cow::Borrowed(_)`.
        assert!(matches!(input, Cow::Borrowed(_)));
    }

    #[test]
    fn owned_no_mutation() {
        // On peut aussi passer `vec` sans `&` pour que `Cow` le possède directement. Dans
        // ce cas, aucune mutation ne se produit (tous les nombres sont déjà absolus) et donc
        // aucun clone. Mais le résultat est toujours possédé car il n'a jamais été
        // emprunté ou muté.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Remplace `todo!()` par `Cow::Owned(_)` ou `Cow::Borrowed(_)`.
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn owned_mutation() {
        // Bien sûr, c'est aussi le cas si une mutation se produit (pas tous
        // les nombres sont absolus). Dans ce cas, l'appel à `to_mut()` dans la
        // fonction `abs_all` renvoie une référence aux mêmes données qu'avant.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Remplace `todo!()` par `Cow::Owned(_)` ou `Cow::Borrowed(_)`.
        assert!(matches!(input, Cow::Owned(_)));
    }
}

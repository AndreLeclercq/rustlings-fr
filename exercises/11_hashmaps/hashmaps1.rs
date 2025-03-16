// Un panier de fruits sous forme de hash map (table de hachage) doit être défini. La clé
// représente le nom du fruit et la valeur représente combien de ce
// fruit particulier se trouve dans le panier. Tu dois mettre au moins 3 différents
// types de fruits (par exemple pomme, banane, mangue) dans le panier et le nombre total
// de tous les fruits doit être au moins 5.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: Déclare la hash map.
    // let mut basket =

    // Deux bananes sont déjà là pour toi :)
    basket.insert(String::from("banane"), 2);

    // TODO: Mets plus de fruits dans ton panier.

    basket
}

fn main() {
    // Tu peux expérimenter ici si tu veux.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}

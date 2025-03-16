// Lors de l'exécution d'opérations sur des éléments au sein d'une collection, les itérateurs (iterators) 
// sont essentiels. Ce module t'aide à te familiariser avec la structure de l'utilisation d'un
// itérateur et comment parcourir les éléments d'une collection itérable.

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterateurs() {
        let mes_fruits_preferes = ["banane", "pomme cannelle", "avocat", "pêche", "framboise"];

        // Crée un itérateur sur le tableau.
        let mut iterateur_fruits_preferes = mes_fruits_preferes.iter();

        assert_eq!(iterateur_fruits_preferes.next(), Some(&"banane"));
        assert_eq!(iterateur_fruits_preferes.next(), Some(&"pomme cannelle"));
        assert_eq!(iterateur_fruits_preferes.next(), Some(&"avocat"));
        assert_eq!(iterateur_fruits_preferes.next(), Some(&"pêche"));
        assert_eq!(iterateur_fruits_preferes.next(), Some(&"framboise"));
        assert_eq!(iterateur_fruits_preferes.next(), None);
        //                                     ^^^^ a atteint la fin
    }
}

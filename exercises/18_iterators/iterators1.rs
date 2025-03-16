// Lors des opérations sur les éléments d'une collection, les itérateurs (iterators) 
// sont essentiels. Ce module t'aide à te familiariser avec la structure 
// d'utilisation d'un itérateur et comment parcourir les éléments d'une collection.

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterateurs() {
        let mes_fruits_preferes = ["banane", "pomme cannelle", "avocat", "pêche", "framboise"];

        // TODO : Crée un itérateur sur le tableau.
        let mut iterateur_fruits_preferes = todo!();

        assert_eq!(iterateur_fruits_preferes.next(), Some(&"banane"));
        assert_eq!(iterateur_fruits_preferes.next(), todo!()); // TODO : Remplace `todo!()`
        assert_eq!(iterateur_fruits_preferes.next(), Some(&"avocat"));
        assert_eq!(iterateur_fruits_preferes.next(), todo!()); // TODO : Remplace `todo!()`
        assert_eq!(iterateur_fruits_preferes.next(), Some(&"framboise"));
        assert_eq!(iterateur_fruits_preferes.next(), todo!()); // TODO : Remplace `todo!()`
    }
}

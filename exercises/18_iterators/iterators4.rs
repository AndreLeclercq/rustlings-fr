fn factorial(num: u64) -> u64 {
    // TODO : Complète cette fonction pour renvoyer la factorielle de `num` 
    // définie comme `1 * 2 * 3 * … * num`.
    // https://fr.wikipedia.org/wiki/Factorielle
    //
    // Ne pas utiliser :
    // - retours anticipés (en utilisant explicitement le mot-clé `return`)
    // Essaie de ne pas utiliser :
    // - boucles impératives (for/while)
    // - variables supplémentaires
    // Pour un défi supplémentaire, n'utilise pas :
    // - récursivité
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}

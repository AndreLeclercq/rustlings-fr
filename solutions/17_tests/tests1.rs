// Les tests sont importants pour s'assurer que ton code fait bien ce que tu penses qu'il devrait faire.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    // Lors de l'écriture de tests unitaires (unit tests), il est courant d'importer tout ce qui vient du module parent (`super`) en utilisant un joker (*).
    use super::*;

    #[test]
    fn tu_peux_affirmer() {
        assert!(is_even(0));
        assert!(!is_even(-1));
        //      ^ Tu peux affirmer `false` en utilisant l'opérateur de négation `!`.
    }
}

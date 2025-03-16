// Les tests sont importants pour s'assurer que ton code fait ce que tu penses qu'il devrait faire.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    // TODO : Importe `is_even`. Tu peux utiliser un caractère générique (wildcard) pour importer tout ce qui est dans le module parent.

    #[test]
    fn tu_peux_verifier() {
        // TODO : Teste la fonction `is_even` avec quelques valeurs.
        assert!();
        assert!();
    }
}

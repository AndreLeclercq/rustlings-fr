fn animal_habitat(animal: &str) -> &str {
    // TODO: Corrige l'erreur de compilation dans l'instruction ci-dessous.
    let identifier = if animal == "crabe" {
        1
    } else if animal == "spermophile" {
        2.0
    } else if animal == "serpent" {
        3
    } else {
        "Inconnu"
    };

    // Ne change pas l'expression ci-dessous !
    if identifier == 1 {
        "Plage"
    } else if identifier == 2 {
        "Terrier"
    } else if identifier == 3 {
        "Désert"
    } else {
        "Inconnu"
    }
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

// Ne modifie pas les tests !
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("spermophile"), "Terrier")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("serpent"), "Désert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crabe"), "Plage")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaure"), "Inconnu")
    }
}

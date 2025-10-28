fn animal_habitat(animal: &str) -> &str {
    let identifier = if animal == "crabe" {
        1
    } else if animal == "spermophile" {
        2
    } else if animal == "serpent" {
        3
    } else {
        // Un identifiant non utilisé.
        4
    };

    // Normalement on utiliserait une enum (énumération) en Rust.
    // Mais nous n'avons pas encore abordé les enums.
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
    // Tu peux faire des tests ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_gaufre_vit_dans_un_terrier() {
        assert_eq!(animal_habitat("spermophile"), "Terrier")
    }

    #[test]
    fn le_serpent_vit_dans_le_desert() {
        assert_eq!(animal_habitat("serpent"), "Désert")
    }

    #[test]
    fn le_crabe_vit_sur_la_plage() {
        assert_eq!(animal_habitat("crabe"), "Plage")
    }

    #[test]
    fn animal_inconnu() {
        assert_eq!(animal_habitat("dinosaure"), "Inconnu")
    }
}

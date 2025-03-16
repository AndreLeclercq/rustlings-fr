// Calcule la puissance de 2 en utilisant un décalage de bits (bit shift).
// `1 << n` est équivalent à "2 à la puissance de n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tu_peux_verifier_egalite() {
        // TODO : Teste la fonction `power_of_2` avec quelques valeurs.
        assert_eq!();
        assert_eq!();
        assert_eq!();
        assert_eq!();
    }
}

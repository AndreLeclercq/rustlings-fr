// AsRef et AsMut permettent des conversions peu coûteuses de référence à référence. En savoir plus
// sur ces traits à https://doc.rust-lang.org/std/convert/trait.AsRef.html et
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectivement.

// Obtiens le nombre d'octets (pas de caractères) dans l'argument donné
// (`.len()` renvoie le nombre d'octets dans une chaîne).
// TODO: Ajoute le trait `AsRef` de manière appropriée comme contrainte de trait.
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().len()
}

// Obtiens le nombre de caractères (pas d'octets) dans l'argument donné.
// TODO: Ajoute le trait `AsRef` de manière appropriée comme contrainte de trait.
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Élève un nombre au carré en utilisant `as_mut()`.
// TODO: Ajoute la contrainte de trait appropriée.
fn num_sq<T>(arg: &mut T) {
    // TODO: Implémente le corps de la fonction.
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}

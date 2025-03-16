trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implémente le trait `AppendBar` pour un vecteur de chaînes de caractères.
// `append_bar` doit ajouter la chaîne "Bar" dans le vecteur.

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}

// Dans cet exercice, tu vas découvrir quelques avantages uniques 
// que les itérateurs (iterators) peuvent offrir.

// TODO : Complète la fonction `capitalize_first`.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => todo!(),
    }
}

// TODO : Applique la fonction `capitalize_first` à un tableau de tranches (slices) de chaînes.
// Renvoie un vecteur de chaînes.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // ???
}

// TODO : Applique à nouveau la fonction `capitalize_first` à un tableau de tranches 
// de chaînes. Renvoie une chaîne unique.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // ???
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_succes() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_vide() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iteration_vecteur_chaines() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iteration_en_chaine() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}

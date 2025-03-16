// Le compilateur Rust a besoin de savoir comment vérifier si les références fournies sont
// valides, afin de pouvoir informer le programmeur si une référence risque de
// sortir de sa portée (scope) avant d'être utilisée. Rappelle-toi, les références sont des emprunts (borrows) et ne
// possèdent pas leurs propres données. Que se passe-t-il si leur propriétaire (owner) sort de sa portée ?

// TODO: Corrige l'erreur du compilateur en mettant à jour la signature de la fonction.
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}

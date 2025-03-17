// Ceci est un quiz pour les sections suivantes:
// - Strings (Chaînes de caractères)
// - Vecs (Vecteurs)
// - Move semantics (Sémantique de déplacement)
// - Modules
// - Enums (Énumérations)
//
// Construisons une petite machine sous forme de fonction. En entrée, nous allons
// donner une liste de chaînes et de commandes. Ces commandes déterminent quelle action
// sera appliquée à la chaîne. Ce peut être:
// - Mettre la chaîne en majuscules
// - Supprimer les espaces en début et fin de chaîne
// - Ajouter "bar" à la chaîne un nombre spécifié de fois
//
// La forme exacte sera:
// - L'entrée sera un Vecteur de tuples de longueur 2,
//   le premier élément est la chaîne, le second est la commande.
// - L'élément de sortie sera un vecteur de chaînes.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complète la fonction comme décrit ci-dessus.
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // Tu peux expérimenter ici si tu veux.
}

#[cfg(test)]
mod tests {
    // TODO: Que devons-nous importer pour avoir `transformer` dans la portée?
    // use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}

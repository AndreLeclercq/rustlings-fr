// TODO: Cette fonction refuse de générer du texte pour un nametag si
// tu lui passes une chaîne vide. Ce serait plus sympa si elle expliquait quel est le problème
// au lieu de simplement renvoyer `None`. Heureusement, Rust possède une structure
// similaire à `Option` qui peut être utilisée pour exprimer des conditions d'erreur. Change
// la signature et le corps de la fonction pour renvoyer `Result<String, String>` au lieu
// de `Option<String>`.
fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // Les noms vides ne sont pas autorisés
        None
    } else {
        Some(format!("Salut ! Je m'appelle {name}"))
    }
}

fn main() {
    // Tu peux expérimenter ici si tu veux.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Salut ! Je m'appelle Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Les noms vides ne sont pas autorisés"),
        );
    }
}

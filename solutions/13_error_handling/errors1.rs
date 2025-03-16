fn generate_nametag_text(name: String) -> Result<String, String> {
    //                                    ^^^^^^         ^^^^^^
    if name.is_empty() {
        // `Err(String)` au lieu de `None`.
        Err("Les noms vides ne sont pas autorisés".to_string())
    } else {
        // `Ok` au lieu de `Some`.
        Ok(format!("Salut ! Je m'appelle {name}"))
    }
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genere_texte_nametag_pour_un_nom_non_vide() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Salut ! Je m'appelle Beyoncé"),
        );
    }

    #[test]
    fn explique_pourquoi_generation_nametag_text_echoue() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Les noms vides ne sont pas autorisés"),
        );
    }
}

trait Licensed {
    // TODO: Ajoute une implémentation par défaut pour `licensing_info` de sorte que
    // les implémenteurs comme les deux structs ci-dessous puissent partager ce comportement
    // par défaut sans répéter la fonction.
    // Les informations de licence par défaut devraient être la chaîne "Default license".
    fn licensing_info(&self) -> String;
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Ne modifie pas cette ligne.
impl Licensed for OtherSoftware {} // Ne modifie pas cette ligne.

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}

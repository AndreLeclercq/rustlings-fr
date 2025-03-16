// Les structs contiennent des données, mais peuvent aussi avoir de la logique. Dans cet exercice, nous avons
// défini la struct `Package`, et nous voulons tester la logique qui y est attachée.

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        if weight_in_grams < 10 {
            // Ce n'est pas comme ça qu'on doit gérer les erreurs en Rust, mais nous
            // apprendrons la gestion des erreurs plus tard.
            panic!("Impossible d'expédier un colis avec un poids inférieur à 10 grammes");
        }

        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    // TODO: Ajoute le bon type de retour à la signature de la fonction.
    fn is_international(&self) {
        // TODO: Lis les tests qui utilisent cette méthode pour déterminer quand un colis
        // est considéré comme international.
    }

    // TODO: Ajoute le bon type de retour à la signature de la fonction.
    fn get_fees(&self, cents_per_gram: u32) {
        // TODO: Calcule les frais du colis.
    }
}

fn main() {
    // Tu peux expérimenter ici si tu veux.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Espagne");
        let recipient_country = String::from("Autriche");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Espagne");
        let recipient_country = String::from("France");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Espagne");
        let recipient_country = String::from("Espagne");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}

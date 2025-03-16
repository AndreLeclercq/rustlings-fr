struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Ne change pas cette fonction.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Retourner un `Result` serait mieux ici. Mais on veut apprendre
            // à tester des fonctions qui peuvent paniquer.
            panic!("La largeur et la hauteur du rectangle doivent être positives");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largeur_et_hauteur_correctes() {
        // TODO : Ce test doit vérifier si le rectangle a la taille que nous
        // passons à son constructeur.
        let rect = Rectangle::new(10, 20);
        assert_eq!(todo!(), 10); // Vérifie la largeur
        assert_eq!(todo!(), 20); // Vérifie la hauteur
    }

    // TODO : Ce test doit vérifier si le programme panique quand on essaie de créer
    // un rectangle avec une largeur négative.
    #[test]
    fn largeur_negative() {
        let _rect = Rectangle::new(-10, 10);
    }

    // TODO : Ce test doit vérifier si le programme panique quand on essaie de créer
    // un rectangle avec une hauteur négative.
    #[test]
    fn hauteur_negative() {
        let _rect = Rectangle::new(10, -10);
    }
}

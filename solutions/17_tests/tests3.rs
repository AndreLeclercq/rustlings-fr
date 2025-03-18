struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Ne change pas cette fonction.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Retourner un `Result` serait préférable ici. Mais on veut apprendre
            // à tester des fonctions qui peuvent provoquer une panique (panic).
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
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // Vérifie la largeur
        assert_eq!(rect.height, 20); // Vérifie la hauteur
    }

    #[test]
    #[should_panic] // Cet attribut vérifie que le test provoque une panique.
    fn largeur_negative() {
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic] // Cet attribut vérifie que le test provoque une panique.
    fn hauteur_negative() {
        let _rect = Rectangle::new(10, -10);
    }
}

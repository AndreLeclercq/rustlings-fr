// Ce quiz teste:
// - Generics (Génériques)
// - Traits
//
// Une école magique imaginaire a un nouveau système de génération de bulletin scolaire écrit
// en Rust! Actuellement, le système ne prend en charge que la création de bulletins où la
// note de l'élève est représentée numériquement (ex. 1.0 -> 5.5). Cependant, l'école
// délivre également des notes alphabétiques (A+ -> F-) et doit pouvoir
// imprimer les deux types de bulletins!
//
// Fais les modifications nécessaires dans la structure `ReportCard` et dans le bloc
// d'implémentation pour prendre en charge les bulletins alphabétiques en plus des numériques.

// TODO: Ajuste la structure comme décrit ci-dessus.
struct ReportCard {
    grade: f32,
    student_name: String,
    student_age: u8,
}

// TODO: Ajuste le bloc d'implémentation comme décrit ci-dessus.
impl ReportCard {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // Tu peux expérimenter ici si tu veux.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}

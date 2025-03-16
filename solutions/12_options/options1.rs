// Cette fonction retourne la quantité de glace restante dans le frigo.
// S'il est avant 22h00, alors il reste 5 boules. À 22h00,
// quelqu'un mange tout, donc il ne reste plus de glace (valeur 0). Retourne `None` si
// `hour_of_day` est supérieur à 23.
fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    match hour_of_day {
        0..=21 => Some(5),
        22..=23 => Some(0),
        _ => None,
    }
}

fn main() {
    // Tu peux expérimenter ici si tu veux.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // Utiliser `unwrap` est acceptable dans un test.
        let icecreams = maybe_icecream(12).unwrap();

        assert_eq!(icecreams, 5);
    }

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(24), None);
        assert_eq!(maybe_icecream(25), None);
    }
}

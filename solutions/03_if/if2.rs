fn picky_eater(food: &str) -> &str {
    if food == "fraise" {
        "Miam !"
    } else if food == "patate" {
        "Je suppose que je peux manger ça."
    } else {
        "Non merci !"
    }
}

fn main() {
    // Tu peux faire des tests ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        assert_eq!(picky_eater("fraise"), "Miam !");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("patate"), "Je suppose que je peux manger ça.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("brocoli"), "Non merci !");
        assert_eq!(picky_eater("bonbons"), "Non merci !");
        assert_eq!(picky_eater("n'importe quoi"), "Non merci !");
    }
}

// Ce magasin fait une promotion : si le prix est un nombre pair, tu obtiens 10
// Rustbucks de réduction, mais si c'est un nombre impair, c'est 3 Rustbucks de réduction.
// Ne t'inquiète pas pour le corps des fonctions, nous sommes seulement intéressés par
// les signatures pour l'instant.

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: Corrige la signature de la fonction.
fn sale_price(price: i64) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Ton prix en promotion est {}", sale_price(original_price));
}

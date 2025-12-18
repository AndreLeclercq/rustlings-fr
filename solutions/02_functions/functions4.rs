fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// Le type de retour doit toujours être annoté.
fn sale_price(price: i64) -> i64 {
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

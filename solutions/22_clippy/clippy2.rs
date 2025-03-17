fn main() {
    let mut res = 42;
    let option = Some(12);
    // Utilise `if-let` au lieu de l'itération.
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}

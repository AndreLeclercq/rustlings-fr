fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    // Ici, les deux réponses fonctionnent.
    // `.into()` convertit un type en un type attendu.
    // S'il est appelé là où `String` est attendu, il convertira `&str` en `String`.
    string("nice weather".into());
    // Mais s'il est appelé là où `&str` est attendu, alors `&str` reste `&str` puisqu'aucune conversion n'est nécessaire.
    // Si tu supprimes la ligne `#[allow(…)]`, alors Clippy te dira de supprimer `.into()` ci-dessous car c'est une conversion inutile.
    #[allow(clippy::useless_conversion)]
    string_slice("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // ATTENTION : C'est de l'indexation d'octets, pas d'indexation de caractères.
    // L'indexation de caractères peut être faite en utilisant `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

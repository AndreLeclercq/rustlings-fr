fn current_favorite_color() -> String {
    // Équivalent à `String::from("blue")`
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("Ma couleur préférée actuelle est {answer}");
}

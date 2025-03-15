fn main() {
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabétique !");
    } else if my_first_initial.is_numeric() {
        println!("Numérique !");
    } else {
        println!("Ni alphabétique ni numérique !");
    }

    // Example avec un emoji.
    let your_character = '🦀';

    if your_character.is_alphabetic() {
        println!("Alphabétique !");
    } else if your_character.is_numeric() {
        println!("Numérique !");
    } else {
        println!("Ni alphabétique ni numérique !");
    }
}

// Ne change pas cette fonction.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Corrige l'erreur du compilateur en déplaçant une ligne.

    let string1 = String::from("une longue chaîne est longue");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
    }
    println!("La plus longue chaîne est '{result}'");
}

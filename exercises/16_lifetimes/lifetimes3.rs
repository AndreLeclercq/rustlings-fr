// Les durées de vie (lifetimes) sont également nécessaires quand les structs contiennent des références.

// TODO: Corrige les erreurs du compilateur concernant la struct.
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} par {}", book.title, book.author);
}

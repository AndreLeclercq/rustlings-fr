// Les durées de vie (lifetimes) sont aussi nécessaires quand les structs contiennent des références.

struct Book<'a> {
    //     ^^^^ annotation de durée de vie ajoutée
    author: &'a str,
    //       ^^
    title: &'a str,
    //      ^^
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} par {}", book.title, book.author);
}

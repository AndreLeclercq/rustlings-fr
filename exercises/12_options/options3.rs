#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Corrige l'erreur du compilateur en ajoutant quelque chose à cette instruction match.
    match optional_point {
        Some(p) => println!("Les coordonnées sont {},{}", p.x, p.y),
        _ => panic!("Pas de correspondance !"),
    }

    println!("{optional_point:?}"); // Ne change pas cette ligne.
}

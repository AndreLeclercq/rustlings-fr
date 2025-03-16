#[derive(Debug)]
struct Point {
   x: i32,
   y: i32,
}

fn main() {
   let optional_point = Some(Point { x: 100, y: 200 });

   // Solution 1 : Matching sur l'`Option` (pas `&Option`) mais sans déplacer
   // la valeur hors de la variante `Some`.
   match optional_point {
       Some(ref p) => println!("Les coordonnées sont {},{}", p.x, p.y),
       //   ^^^ ajouté
       _ => panic!("Pas de correspondance !"),
   }

   // Solution 2 : Matching sur une référence (`&Option`) en ajoutant `&` avant
   // `optional_point`.
   match &optional_point {
       //^ ajouté
       Some(p) => println!("Les coordonnées sont {},{}", p.x, p.y),
       _ => panic!("Pas de correspondance !"),
   }

   println!("{optional_point:?}");
}

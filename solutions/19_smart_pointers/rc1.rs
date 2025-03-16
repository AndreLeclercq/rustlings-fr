// Dans cet exercice, nous voulons exprimer le concept de propriétaires multiples via le
// type `Rc<T>`. C'est un modèle de notre système solaire - il y a un type `Sun` (Soleil) et
// plusieurs `Planet`s (Planètes). Les planètes prennent possession du soleil, indiquant qu'elles
// tournent autour du soleil.

use std::rc::Rc;

#[derive(Debug)]
struct Sun;

#[derive(Debug)]
enum Planet {
   Mercury(Rc<Sun>),
   Venus(Rc<Sun>),
   Earth(Rc<Sun>),
   Mars(Rc<Sun>),
   Jupiter(Rc<Sun>),
   Saturn(Rc<Sun>),
   Uranus(Rc<Sun>),
   Neptune(Rc<Sun>),
}

impl Planet {
   fn details(&self) {
       println!("Salut de {self:?}!");
   }
}

fn main() {
   // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn rc1() {
       let sun = Rc::new(Sun);
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 1 référence

       let mercury = Planet::Mercury(Rc::clone(&sun));
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 2 références
       mercury.details();

       let venus = Planet::Venus(Rc::clone(&sun));
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 3 références
       venus.details();

       let earth = Planet::Earth(Rc::clone(&sun));
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 4 références
       earth.details();

       let mars = Planet::Mars(Rc::clone(&sun));
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 5 références
       mars.details();

       let jupiter = Planet::Jupiter(Rc::clone(&sun));
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 6 références
       jupiter.details();

       let saturn = Planet::Saturn(Rc::clone(&sun));
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 7 références
       saturn.details();

       // TODO
       let uranus = Planet::Uranus(Rc::clone(&sun));
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 8 références
       uranus.details();

       // TODO
       let neptune = Planet::Neptune(Rc::clone(&sun));
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 9 références
       neptune.details();

       assert_eq!(Rc::strong_count(&sun), 9);

       drop(neptune);
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 8 références

       drop(uranus);
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 7 références

       drop(saturn);
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 6 références

       drop(jupiter);
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 5 références

       drop(mars);
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 4 références

       drop(earth);
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 3 références

       drop(venus);
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 2 références

       drop(mercury);
       println!("nombre de références = {}", Rc::strong_count(&sun)); // 1 référence

       assert_eq!(Rc::strong_count(&sun), 1);
   }
}

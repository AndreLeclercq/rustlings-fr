// La conversion de type en Rust se fait via l'utilisation de l'opérateur `as`.
// Note que l'opérateur `as` n'est pas seulement utilisé pour la conversion de type. Il aide
// aussi à renommer les imports.

fn average(values: &[f64]) -> f64 {
   let total = values.iter().sum::<f64>();
   // TODO: Fais une conversion avant de diviser.
   total / values.len()
}

fn main() {
   let values = [3.5, 0.3, 13.0, 11.7];
   println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn returns_proper_type_and_value() {
       assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
   }
}

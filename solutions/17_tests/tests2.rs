// Calcule la puissance de 2 en utilisant un décalage de bits (bit shift).
// `1 << n` est équivalent à "2 à la puissance de n".
fn power_of_2(n: u8) -> u64 {
   1 << n
}

fn main() {
   // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn tu_peux_affirmer_egal() {
       assert_eq!(power_of_2(0), 1);
       assert_eq!(power_of_2(1), 2);
       assert_eq!(power_of_2(2), 4);
       assert_eq!(power_of_2(3), 8);
   }
}

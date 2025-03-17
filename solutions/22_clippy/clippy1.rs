// L'outil Clippy est une collection de lints pour analyser ton code afin que tu puisses
// repérer les erreurs courantes et améliorer ton code Rust.
//
// Pour ces exercices, le code ne compilera pas quand il y a des avertissements Clippy.
// Consulte les suggestions de Clippy dans la sortie pour résoudre l'exercice.

use std::f32::consts::PI;

fn main() {
    // Utilise la constante `PI` plus précise.
    let pi = PI;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("L'aire d'un cercle de rayon {radius:.2} est {area:.5}");
}

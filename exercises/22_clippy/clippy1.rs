// L'outil Clippy est une collection de lints (analyseurs) pour analyser ton code afin que tu puisses
// repérer les erreurs courantes et améliorer ton code Rust.
//
// Pour ces exercices, le code ne compilera pas lorsqu'il y a des avertissements Clippy.
// Vérifie les suggestions de Clippy dans la sortie pour résoudre l'exercice.

fn main() {
    // TODO: Corrige le lint Clippy dans cette ligne.
    let pi = 3.14;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("L'aire d'un cercle de rayon {radius:.2} est {area:.5}");
}

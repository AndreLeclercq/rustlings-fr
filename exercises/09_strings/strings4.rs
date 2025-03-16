// Les appels à cette fonction devraient être remplacés par des appels à `string_slice` ou `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
   println!("{arg}");
}

fn string(arg: String) {
   println!("{arg}");
}

// TODO: Voici plusieurs valeurs - certaines sont `String`, d'autres sont `&str`.
// Ta tâche est de remplacer `placeholder(…)` par soit `string_slice(…)`
// soit `string(…)` selon ce que tu penses que chaque valeur est.
fn main() {
   placeholder("blue");

   placeholder("red".to_string());

   placeholder(String::from("hi"));

   placeholder("rust is fun!".to_owned());

   placeholder("nice weather".into());

   placeholder(format!("Interpolation {}", "Station"));

   // ATTENTION: Ceci est une indexation d'octets, pas une indexation de caractères.
   // L'indexation de caractères peut être faite en utilisant `s.chars().nth(INDEX)`.
   placeholder(&String::from("abc")[0..1]);

   placeholder("  hello there ".trim());

   placeholder("Happy Monday!".replace("Mon", "Tues"));

   placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

// TODO: Ajoute le type manquant de l'argument `num` après les deux-points `:`.
fn call_me(num:) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}

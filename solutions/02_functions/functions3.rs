fn call_me(num: u8) {
    for i in 0..num {
        println!("Dring ! Appel numéro {}", i + 1);
    }
}

fn main() {
    // `call_me` attend un argument.
    call_me(5);
}

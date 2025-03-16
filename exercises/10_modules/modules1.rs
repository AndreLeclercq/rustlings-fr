// TODO: Corrige l'erreur du compilateur concernant l'appel d'une fonction private (privée).
mod sausage_factory {
    // Ne laisse personne en dehors de ce module voir ceci !
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("saucisse !");
    }
}

fn main() {
    sausage_factory::make_sausage();
}

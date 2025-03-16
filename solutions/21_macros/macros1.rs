macro_rules! my_macro {
    () => {
        println!("Découvre ma macro !");
    };
}

fn main() {
    my_macro!();
    //      ^
}

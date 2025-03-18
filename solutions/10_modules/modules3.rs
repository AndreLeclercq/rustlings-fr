use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!(
            "Le 1970-01-01 00:00:00 UTC était il y a {} secondes !",
            n.as_secs()
        ),
        Err(_) => panic!("SystemTime avant UNIX EPOCH !"),
    }
}

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        //        ^^^ c'est important
        self.push(String::from("Bar"));
        self
    }
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}

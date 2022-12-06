pub mod aoc {

    #[derive(Debug, Copy, Clone)]
    pub struct Container {
        pub value: char,
    }

    impl Container {
        pub fn new(input: &str) -> Self {
            Container {
                value: input.chars().nth(1).unwrap(),
            }
        }

        pub fn is_empty(&self) -> bool {
            self.value == ' '
        }
    }
}

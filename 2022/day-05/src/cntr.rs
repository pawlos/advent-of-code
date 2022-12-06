/*
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
 */
pub mod aoc {
    use crate::Container;

    #[derive(Debug)]
    pub struct StackOfContainers {
        stack: Vec<Container>,
    }

    impl StackOfContainers {
        pub fn new() -> Self {
            StackOfContainers { stack: Vec::new() }
        }

        pub fn push(&mut self, container: Container) {
            if !container.is_empty() {
                self.stack.push(container)
            }
        }

        pub fn pop(&mut self) -> Container {
            self.stack.pop().unwrap()
        }

        pub fn pop_many(&mut self, cnt: usize) -> Vec<Container> {
            let mut result = Vec::new();
            for _ in 0..cnt {
                result.push(self.pop())
            }
            result
        }

        pub fn push_many(&mut self, vec: Vec<Container>) {
            for elem in vec {
                self.push(elem)
            }
        }
    }
}

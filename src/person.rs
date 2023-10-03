pub mod Person {
    pub struct Person {
        pub name: String,
        age: usize,
    }

    impl Person {
        pub fn create(name: String, age: usize) -> Person {
            Person { name, age }
        }

        pub fn print(&self) {
            println!("name: {} and age: {}", self.name, self.age);
        }
    }
}

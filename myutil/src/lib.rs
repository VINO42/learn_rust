 pub mod myutil {

    pub fn install(name: String, age: i32) -> Person {
        Person {
            name,
            age,
        }
    }

    pub struct Person {
        name: String,
        age: i32,
    }

    impl Person {
        pub fn say(&self) {
            println! {"hello world my name is{},and Im {} old", &self.name, &self.age};
        }
    }
}

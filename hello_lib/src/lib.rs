mod advanced_greeter;

pub struct Greeter {
    greeting: String,
}

impl Greeter {
    pub fn new(greeting: &str) -> Greeter {
        Greeter {
            greeting: greeting.to_string(),
        }
    }

    pub fn greet(&self, thing: &str) {
        println!("{} {}", &self.greeting, thing);
    }

    pub fn x() {
        use itertools::Itertools;

        let it = (1..7).interleave(vec![-1, -2]);
        println!("{:?}", it);
    }

    pub fn test() {
        Self::x()
    }
}

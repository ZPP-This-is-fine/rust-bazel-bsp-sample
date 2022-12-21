#[derive(Debug, PartialEq, Eq)]
pub struct AdvancedGreeter(String, i32);

impl AdvancedGreeter {
    pub fn new(s: String, i: i32) -> Self {
        Self(s, i)
    }

    pub fn greet(&self, thing: &str) {
        for i in 0..self.1 {
            println!("{} {}", self.0, thing);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AdvancedGreeter;

    #[test]
    fn new() {
        assert_eq!(AdvancedGreeter::new("lol".to_string(), 2137), AdvancedGreeter("lol".to_string(), 2137))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct AdvancedGreeter(String, i32);

impl AdvancedGreeter {
    #[allow(dead_code)]
    pub fn new(s: String, i: i32) -> Self {
        Self(s, i)
    }

    #[allow(dead_code)]
    pub fn greet(&self, thing: &str) {
        for _i in 0..self.1 {
            println!("{} {}", self.0, thing);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AdvancedGreeter;

    #[test]
    fn new() {
        assert_eq!(
            AdvancedGreeter::new("lol".to_string(), 2137),
            AdvancedGreeter("lol".to_string(), 2137)
        )
    }
}

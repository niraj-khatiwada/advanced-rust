pub fn greet(name: &str) -> String {
    format!("Hello {}", name)
}

#[derive(Debug, Clone)]
pub struct Person {
    name: String,
}

impl Person {
    pub fn new(name: &str) -> Self {
        Person {
            name: String::from(name),
        }
    }

    pub fn get_name(&self) -> String {
        String::from(&self.name)
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Hello Niraj"), greet("Niraj"))
    }
}

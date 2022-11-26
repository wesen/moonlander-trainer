use crate::generators::{TypingPattern, TypingPatternGenerator};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct SingleStringGenerator {
    pub name: String,
    pub pattern: String,
}

impl SingleStringGenerator {
    pub fn new(name: &str, pattern: &str) -> SingleStringGenerator {
        SingleStringGenerator {
            name: name.to_string(),
            pattern: pattern.to_string(),
        }
    }
}

impl TypingPatternGenerator for SingleStringGenerator {
    fn generate(&self) -> TypingPattern {
        TypingPattern {
            name: self.name.clone(),
            pattern: self.pattern.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ListOfPatternsGenerator {
    pub name: String,
    pub patterns: Vec<Rc<dyn TypingPatternGenerator>>,
    pub delimiter: String,
}

impl ListOfPatternsGenerator {
    pub fn new(
        name: &'static str,
        children: Vec<Rc<dyn TypingPatternGenerator>>,
        config: HashMap<&str, String>,
    ) -> Self {
        let delimiter = config
            .get("delimiter")
            .unwrap_or(&String::from(" "))
            .to_string();
        ListOfPatternsGenerator {
            name: name.to_string(),
            patterns: children,
            delimiter,
        }
    }
}

impl TypingPatternGenerator for ListOfPatternsGenerator {
    fn generate(&self) -> TypingPattern {
        let mut generated_patterns: Vec<TypingPattern> = Vec::new();
        for child in &self.patterns {
            generated_patterns.push(child.generate());
        }
        let pattern: String = generated_patterns
            .iter()
            .map(|x| x.pattern.clone())
            .collect::<Vec<String>>()
            .join(&self.delimiter);

        TypingPattern {
            name: self.name.clone(),
            pattern,
        }
    }
}

fn test() {
    let foo = Box::new(SingleStringGenerator::new("foo", "foo"));
    let foo_ = RefCell::new(foo);
    foo_.generate();
}

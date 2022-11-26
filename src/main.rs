mod hid;

extern crate hidapi;

use std::collections::HashMap;
use std::fmt::Debug;
use rand::prelude::*;

#[derive(Debug)]
struct TypingPattern {
    pub name: String,
    pub pattern: String,
}

trait TypingPatternGenerator {
    fn generate(&self) -> TypingPattern;
}

#[derive(Debug)]
struct ListOfPatternsGenerator {
    pub name: String,
    pub patterns: Vec<Box<dyn TypingPatternGenerator>>,
    pub delimiter: String,

}

impl ListOfPatternsGenerator {
    fn new(name: &'static str,
           children: Vec<Box<dyn TypingPatternGenerator>>,
           config: HashMap<String, String>) -> Self {
        let delimiter = config.get("delimiter").unwrap_or(&String::from(" ")).to_string();
        ListOfPatternsGenerator {
            name: name.to_string(),
            patterns: children,
            delimiter,
        }
    }
}

impl Debug for Box<dyn TypingPatternGenerator> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Box<dyn TypingPatternGenerator>")
    }
}

impl TypingPatternGenerator for ListOfPatternsGenerator {
    fn generate(&self) -> TypingPattern {
        let mut generated_patterns: Vec<TypingPattern> = Vec::new();
        for child in &self.patterns {
            generated_patterns.push(child.generate());
        }
        let pattern: String = generated_patterns.iter().map(|x| x.pattern.clone()).collect::<Vec<String>>().join(&self.delimiter);

        TypingPattern {
            name: self.name.clone(),
            pattern,
        }
    }
}

#[derive(Debug)]
struct RepeatPatternGenerator {
    pub name: String,
    pub pattern: Box<dyn TypingPatternGenerator>,
    pub count: u32,
    pub delimiter: String,
}

impl RepeatPatternGenerator {
    fn new(name: &'static str,
           child: Box<dyn TypingPatternGenerator>,
           config: HashMap<String, String>) -> Self {
        let count = config.get("count").unwrap_or(&String::from("1")).parse::<u32>().unwrap_or(1);
        let delimiter = config.get("delimiter").unwrap_or(&String::from(" ")).to_string();
        RepeatPatternGenerator {
            name: name.to_string(),
            pattern: child,
            count,
            delimiter,
        }
    }
}

impl TypingPatternGenerator for RepeatPatternGenerator {
    fn generate(&self) -> TypingPattern {
        let mut generated_patterns: Vec<TypingPattern> = Vec::new();
        for _ in 0..self.count {
            generated_patterns.push(self.pattern.generate());
        }
        let pattern: String = generated_patterns.iter().map(|x| x.pattern.clone()).collect::<Vec<String>>().join(&self.delimiter);

        TypingPattern {
            name: self.name.clone(),
            pattern,
        }
    }
}

#[derive(Debug)]
struct NumberPatternGenerator {
    name: &'static str,
    min_length: u32,
    max_length: u32,
}

impl NumberPatternGenerator {
    fn new(name: &'static str,
           _children: Vec<Box<dyn TypingPatternGenerator>>,
           config: HashMap<String, String>) -> Self {
        let mut min_length: u32 = 4;
        let mut max_length: u32 = 8;
        config.get("min_length").map(|s| min_length = s.parse().unwrap());
        config.get("max_length").map(|s| max_length = s.parse().unwrap());
        NumberPatternGenerator {
            name,
            min_length,
            max_length,
        }
    }
}

impl TypingPatternGenerator for NumberPatternGenerator {
    fn generate(&self) -> TypingPattern {
        let mut pattern = String::new();
        let mut rng = thread_rng();
        let length = rng.gen_range(self.min_length..self.max_length);
        for _ in 0..length {
            pattern.push(rng.gen_range(0..10).to_string().chars().next().unwrap());
        }

        TypingPattern {
            name: self.name.to_string(),
            pattern,
        }
    }
}

fn main() {
    let list_of_numbers = Box::new(ListOfPatternsGenerator::new(
        "list_of_numbers",
        vec![Box::new(NumberPatternGenerator::new("number", vec![], HashMap::new()))],
        HashMap::new(),
    ));
    let repeat_numbers = &RepeatPatternGenerator::new(
        "repeat_numbers",
        Box::new(NumberPatternGenerator::new("number", vec![], HashMap::new())),
        HashMap::from([
            ("count".to_string(), "3".to_string()),
            ("delimiter".to_string(), "-".to_string())
        ]),
    );
    println!("{:?}", list_of_numbers.generate().pattern);
    println!("{:?}", repeat_numbers.generate().pattern);
    // hid::hid::test_hidapi();
}

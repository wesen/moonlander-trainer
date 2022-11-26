mod hid;

extern crate hidapi;

use rand::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
struct TypingPattern {
    pub name: String,
    pub pattern: String,
}

trait TypingPatternGenerator: Debug {
    fn generate(&self) -> TypingPattern;
}

#[derive(Debug)]
struct SingleStringGenerator {
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
struct ListOfPatternsGenerator {
    pub name: String,
    pub patterns: Vec<Rc<dyn TypingPatternGenerator>>,
    pub delimiter: String,
}

impl ListOfPatternsGenerator {
    fn new(
        name: &'static str,
        children: Vec<Rc<dyn TypingPatternGenerator>>,
        config: HashMap<String, String>,
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

#[derive(Debug)]
struct WeightedPatternGenerator {
    pub name: String,
    pub patterns: Vec<(f32, Rc<dyn TypingPatternGenerator>)>,
    pub total_weight: f32,
}

impl WeightedPatternGenerator {
    fn new(
        name: &'static str,
        children: Vec<(f32, Rc<dyn TypingPatternGenerator>)>,
        _config: HashMap<String, String>,
    ) -> Self {
        let total_weight = children.iter().map(|x| x.0).sum();
        WeightedPatternGenerator {
            name: name.to_string(),
            patterns: children,
            total_weight,
        }
    }
}

impl TypingPatternGenerator for WeightedPatternGenerator {
    fn generate(&self) -> TypingPattern {
        let mut rng = thread_rng();
        let mut random_number = rng.gen_range(0.0..self.total_weight);
        for (weight, child) in &self.patterns {
            random_number -= weight;
            if random_number <= 0.0 {
                return child.generate();
            }
        }
        panic!("Failed to generate a pattern");
    }
}

#[derive(Debug)]
struct OneOfStringsPatternGenerator {
    pub name: String,
    pub strings: Vec<String>,
}

impl OneOfStringsPatternGenerator {
    fn new(name: &'static str, strings: Vec<&'static str>) -> Self {
        OneOfStringsPatternGenerator {
            name: name.to_string(),
            strings: strings.iter().map(|x| x.to_string()).collect(),
        }
    }
}

impl TypingPatternGenerator for OneOfStringsPatternGenerator {
    fn generate(&self) -> TypingPattern {
        let mut rng = thread_rng();
        let pattern = self.strings.choose(&mut rng).unwrap().clone();
        TypingPattern {
            name: self.name.clone(),
            pattern,
        }
    }
}

// From: https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[derive(Debug)]
struct RepeatPatternGenerator {
    pub name: String,
    pub pattern: Rc<dyn TypingPatternGenerator>,
    pub count: u32,
    pub delimiter: String,
    pub camel_case_strings: bool,
}

impl RepeatPatternGenerator {
    fn new(
        name: &'static str,
        child: Rc<dyn TypingPatternGenerator>,
        config: HashMap<String, String>,
    ) -> Self {
        let count = config
            .get("count")
            .unwrap_or(&String::from("4"))
            .parse::<u32>()
            .unwrap_or(4);
        let delimiter = config
            .get("delimiter")
            .unwrap_or(&String::from(" "))
            .to_string();
        let camel_case_strings = config
            .get("camel_case_strings")
            .unwrap_or(&String::from("false"))
            .parse::<bool>()
            .unwrap_or(false);
        RepeatPatternGenerator {
            name: name.to_string(),
            pattern: child,
            count,
            delimiter,
            camel_case_strings,
        }
    }
}

impl TypingPatternGenerator for RepeatPatternGenerator {
    fn generate(&self) -> TypingPattern {
        let mut generated_patterns: Vec<TypingPattern> = Vec::new();
        for _ in 0..self.count {
            generated_patterns.push(self.pattern.generate());
        }
        let pattern: String = generated_patterns
            .iter()
            .map(|x| {
                if self.camel_case_strings {
                    uppercase_first_letter(&x.pattern)
                } else {
                    x.pattern.clone()
                }
            })
            .collect::<Vec<String>>()
            .join(&self.delimiter);

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
    fn new(name: &'static str, config: HashMap<String, String>) -> Self {
        let mut min_length: u32 = 4;
        let mut max_length: u32 = 8;
        config
            .get("min_length")
            .map(|s| min_length = s.parse().unwrap());
        config
            .get("max_length")
            .map(|s| max_length = s.parse().unwrap());
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
    let number = Rc::new(NumberPatternGenerator::new("number", HashMap::new()));
    let list_of_numbers = Rc::new(ListOfPatternsGenerator::new(
        "list_of_numbers",
        vec![number.clone()],
        HashMap::new(),
    ));
    let repeat_numbers = RepeatPatternGenerator::new(
        "repeat_numbers",
        number.clone(),
        HashMap::from([
            ("count".to_string(), "3".to_string()),
            ("delimiter".to_string(), "-".to_string()),
        ]),
    );
    let camel_cased_symbols = Rc::new(RepeatPatternGenerator::new(
        "camel_cased_symbols",
        Rc::new(OneOfStringsPatternGenerator::new(
            "list_of_symbols",
            vec![
                "previous",
                "next",
                "symbol",
                "factory",
                "creator",
                "generator",
                "abstract",
            ],
        )),
        HashMap::from([
            ("delimiter".to_string(), "".to_string()),
            ("camel_case_strings".to_string(), "true".to_string()),
        ]),
    ));

    let open_paren = Rc::new(SingleStringGenerator::new("open_paren", "("));
    let close_paren = Rc::new(SingleStringGenerator::new("close_paren", ")"));
    let open_bracket = Rc::new(SingleStringGenerator::new("open_bracket", "["));
    let close_bracket = Rc::new(SingleStringGenerator::new("close_bracket", "]"));
    let number_arguments = Rc::new(RepeatPatternGenerator::new(
        "arguments",
        list_of_numbers.clone(),
        HashMap::from([
            ("count".to_string(), "3".to_string()),
            ("delimiter".to_string(), ", ".to_string()),
        ]),
    ));
    let array_deref = Rc::new(ListOfPatternsGenerator::new(
        "array_deref",
        vec![
            camel_cased_symbols.clone(),
            open_bracket.clone(),
            number.clone(),
            close_bracket.clone(),
        ],
        HashMap::from([("delimiter".to_string(), "".to_string())]),
    ));

    let method_call_generator = ListOfPatternsGenerator::new(
        "method_call",
        vec![
            camel_cased_symbols.clone(),
            open_paren.clone(),
            number_arguments.clone(),
            close_paren.clone(),
        ],
        HashMap::from([("delimiter".to_string(), "".to_string())]),
    );

    // hid::hid::test_hidapi();
    println!("{:?}", list_of_numbers.generate().pattern);
    println!("{:?}", repeat_numbers.generate().pattern);
    println!("{:?}", camel_cased_symbols.generate().pattern);
    println!("{:?}", method_call_generator.generate().pattern);
    println!("{:?}", array_deref.generate().pattern);
}

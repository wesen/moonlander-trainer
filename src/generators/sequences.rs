use crate::generators::{TypingPattern, TypingPatternGenerator};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct RepeatPatternGenerator {
    pub name: String,
    pub pattern: Rc<dyn TypingPatternGenerator>,
    pub count: u32,
    pub delimiter: String,
    pub camel_case_strings: bool,
}

impl RepeatPatternGenerator {
    pub fn new(
        name: &'static str,
        child: Rc<dyn TypingPatternGenerator>,
        config: HashMap<&str, String>,
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
                    crate::generators::helpers::uppercase_first_letter(&x.pattern)
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
pub struct RandomRepeatGenerator {
    pub name: String,
    pub pattern: Rc<dyn TypingPatternGenerator>,
    pub min_count: u32,
    pub max_count: u32,
    pub delimiter: String,
    pub camel_case_strings: bool,
}

impl RandomRepeatGenerator {
    pub fn new(
        name: &'static str,
        child: Rc<dyn TypingPatternGenerator>,
        config: HashMap<&str, String>,
    ) -> Self {
        let min_count = config
            .get("min_count")
            .unwrap_or(&String::from("2"))
            .parse::<u32>()
            .unwrap_or(1);
        let max_count = config
            .get("max_count")
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
        RandomRepeatGenerator {
            name: name.to_string(),
            pattern: child,
            min_count,
            max_count,
            delimiter,
            camel_case_strings,
        }
    }
}

impl TypingPatternGenerator for RandomRepeatGenerator {
    fn generate(&self) -> TypingPattern {
        let mut rng = thread_rng();
        let count = rng.gen_range(self.min_count..self.max_count);
        let mut generated_patterns: Vec<TypingPattern> = Vec::new();
        for _ in 0..count {
            generated_patterns.push(self.pattern.generate());
        }
        let pattern: String = generated_patterns
            .iter()
            .map(|x| {
                if self.camel_case_strings {
                    crate::generators::helpers::uppercase_first_letter(&x.pattern)
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

use crate::generators::{TypingPattern, TypingPatternGenerator};
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::rc::Rc;

#[derive(Debug)]
pub struct OneOfStringsPatternGenerator {
    pub name: String,
    pub strings: Vec<String>,
}

impl OneOfStringsPatternGenerator {
    pub fn new(name: &'static str, strings: Vec<&'static str>) -> Self {
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

#[derive(Debug)]
pub struct WeightedPatternGenerator {
    pub name: String,
    pub patterns: Vec<(f32, Rc<dyn TypingPatternGenerator>)>,
    pub total_weight: f32,
}

impl WeightedPatternGenerator {
    pub fn new(name: &'static str, children: Vec<(f32, Rc<dyn TypingPatternGenerator>)>) -> Self {
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

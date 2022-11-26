use crate::generators::randomized::OneOfStringsPatternGenerator;
use crate::generators::sequences::RandomRepeatGenerator;
use crate::generators::simple::{ListOfPatternsGenerator, SingleStringGenerator};
use crate::generators::{TypingPattern, TypingPatternGenerator};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct NumberPatternGenerator {
    name: &'static str,
    min_length: u32,
    max_length: u32,
}

impl NumberPatternGenerator {
    pub fn new(name: &'static str, config: HashMap<&str, String>) -> Self {
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

pub struct CodingGenerators {
    pub open_paren: Rc<dyn TypingPatternGenerator>,
    pub close_paren: Rc<dyn TypingPatternGenerator>,
    pub open_bracket: Rc<dyn TypingPatternGenerator>,
    pub close_bracket: Rc<dyn TypingPatternGenerator>,
    pub open_brace: Rc<dyn TypingPatternGenerator>,
    pub close_brace: Rc<dyn TypingPatternGenerator>,
    pub symbols: Rc<dyn TypingPatternGenerator>,
    pub semicolon: Rc<dyn TypingPatternGenerator>,
    pub number: Rc<dyn TypingPatternGenerator>,
    pub number_list: Rc<dyn TypingPatternGenerator>,
    pub array_deref: Rc<dyn TypingPatternGenerator>,
}

pub fn create_coding_generators() -> CodingGenerators {
    let number = Rc::new(NumberPatternGenerator::new(
        "number",
        HashMap::from([
            ("min_length", "3".to_string()),
            ("max_length", "5".to_string()),
        ]),
    ));
    let list_of_numbers = Rc::new(ListOfPatternsGenerator::new(
        "list_of_numbers",
        vec![number.clone()],
        HashMap::new(),
    ));
    let camel_cased_symbols = Rc::new(RandomRepeatGenerator::new(
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
                "class",
                "interface",
                "function",
                "method",
                "constructor",
                "destructor",
                "getter",
                "setter",
                "property",
                "variable",
            ],
        )),
        HashMap::from([
            ("delimiter", "".to_string()),
            ("min_count", "1".to_string()),
            ("max_count", "3".to_string()),
            ("camel_case_strings", "true".to_string()),
        ]),
    ));

    let open_paren = Rc::new(SingleStringGenerator::new("open_paren", "("));
    let close_paren = Rc::new(SingleStringGenerator::new("close_paren", ")"));
    let open_bracket = Rc::new(SingleStringGenerator::new("open_bracket", "["));
    let close_bracket = Rc::new(SingleStringGenerator::new("close_bracket", "]"));
    let array_deref = Rc::new(ListOfPatternsGenerator::new(
        "array_deref",
        vec![
            camel_cased_symbols.clone(),
            open_bracket.clone(),
            number.clone(),
            close_bracket.clone(),
        ],
        HashMap::from([("delimiter", "".to_string())]),
    ));
    CodingGenerators {
        open_paren,
        close_paren,
        open_bracket,
        close_bracket,
        open_brace: Rc::new(SingleStringGenerator::new("open_brace", "{")),
        close_brace: Rc::new(SingleStringGenerator::new("close_brace", "}")),
        symbols: camel_cased_symbols,
        semicolon: Rc::new(SingleStringGenerator::new("semicolon", ";")),
        number,
        number_list: list_of_numbers,
        array_deref,
    }
}

pub fn create_method_call_generator(
    name: &'static str,
    method_name: Rc<dyn TypingPatternGenerator>,
    open_paren: Rc<dyn TypingPatternGenerator>,
    close_paren: Rc<dyn TypingPatternGenerator>,
    argument: Rc<dyn TypingPatternGenerator>,
    argument_delimiter: String,
    min_arguments: u32,
    max_arguments: u32,
) -> Rc<dyn TypingPatternGenerator> {
    return Rc::new(ListOfPatternsGenerator::new(
        name,
        vec![
            method_name,
            open_paren,
            Rc::new(RandomRepeatGenerator::new(
                "arguments",
                argument,
                HashMap::from([
                    ("delimiter", argument_delimiter),
                    ("min_count", min_arguments.to_string()),
                    ("max_count", max_arguments.to_string()),
                ]),
            )),
            close_paren,
        ],
        HashMap::from([("delimiter", String::from(""))]),
    ));
}

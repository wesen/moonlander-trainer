mod generators;
mod hid;

extern crate hidapi;

use crate::generators::coding::{create_coding_generators, create_method_call_generator};
use crate::generators::sequences::RandomRepeatGenerator;
use generators::sequences::RepeatPatternGenerator;
use generators::simple::ListOfPatternsGenerator;
use generators::TypingPatternGenerator;

use crate::generators::randomized::WeightedPatternGenerator;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let coding_generator = create_coding_generators();
    let number_arguments = Rc::new(RepeatPatternGenerator::new(
        "arguments",
        coding_generator.number_list.clone(),
        HashMap::from([("count", "3".to_string()), ("delimiter", ", ".to_string())]),
    ));
    let method_call_generator = create_method_call_generator(
        "method_call",
        coding_generator.symbols.clone(),
        coding_generator.open_paren.clone(),
        coding_generator.close_paren.clone(),
        number_arguments.clone(),
        ", ".to_string(),
        0,
        2,
    );

    let mut tree_content = Rc::new(RefCell::new(WeightedPatternGenerator::new(
        "tree_content",
        vec![
            (1f32, coding_generator.number.clone()),
            (1f32, coding_generator.array_deref.clone()),
            (1f32, method_call_generator.clone()),
        ],
    )));

    let repeated_subtrees = Rc::new(RandomRepeatGenerator::new(
        "repeated_subtrees",
        tree_content.clone(),
        HashMap::from([
            ("delimiter", ", ".to_string()),
            ("min_count", "1".to_string()),
            ("max_count", "3".to_string()),
        ]),
    ));

    let value = Box::new(ListOfPatternsGenerator::new(
        "tree",
        vec![
            coding_generator.open_bracket.clone(),
            repeated_subtrees.clone(),
            coding_generator.close_bracket.clone(),
        ],
        HashMap::from([("delimiter", "".to_string())]),
    ));
    let mut tree_generator = Rc::new(RefCell::new(value));
    // tree_content.borrow_mut().patterns[1] = (5f32, repeated_subtrees.clone());

    // hid::hid::test_hidapi();
    for _ in 0..100 {
        println!("{}", tree_generator.borrow().generate().pattern);
    }
}

pub(crate) mod coding;
pub(crate) mod helpers;
pub(crate) mod randomized;
pub(crate) mod sequences;
pub(crate) mod simple;

use std::cell::RefCell;
use std::fmt::Debug;

#[derive(Debug)]
pub struct TypingPattern {
    pub name: String,
    pub pattern: String,
}

pub trait TypingPatternGenerator: Debug {
    fn generate(&self) -> TypingPattern;
}

impl<T: ?Sized + TypingPatternGenerator> TypingPatternGenerator for Box<T> {
    fn generate(&self) -> TypingPattern {
        (**self).generate()
    }
}

impl<T: ?Sized + TypingPatternGenerator> TypingPatternGenerator for RefCell<T> {
    fn generate(&self) -> TypingPattern {
        self.borrow().generate()
    }
}

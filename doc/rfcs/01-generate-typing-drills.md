# RFC 01 - Generate typing drills

## Overview 

I want to generate pure text typing drills to help me deliberately practice 
special constructs I encounter a lot in programming languages.

General typing drills oriented towards programming like typing.io
focus on typing full source code files, but not only does this not
really reflect what a programmer does in practice, but specific patterns 
like array indexing or object initialization, which combine numbers, symbols
and strings (often snake_case or camelCase) in specific ways are not repeated
and varied enough to really create proper muscle memory.

Similar to online typing tools like keybr.com, which focus on alphabetic typing,
but really drill in on specific letter combinations, make it much easier to really
become fluent with touch-typing.

## Brainstorm

### Patterns I want to practice

I want to practice patterns like the following:

```javascript
foobarBaz[123]["blabla"]
foo - (123 - bar.baz.test[foo.bar].bla)
if (foo) { }
for (i = 0; i <= len; i++) {}

class foobar {}
class foobar extends bar {}

(foo, bar) => {}
(foo: type, bar: testType, { deconstruct, bar } : { deconstruct: string, bar: number}) => {}
```

Actually, these are not very focused drills either, and the patterns should probably be more restricted, 
there is no need to make them valid programming language statements.

### Generators and patterns

A config file could specify a list of named patterns, 
so that we can easily refer to them when constructing nested generators.
Each generator will create what is called a "pattern", which is the actual combination
of keys to be typed (for starters, just plain strings), as well as a reference to the generator that
created it (for now just the name because I don't really have the patience for rust ownership
magic).

```rust
#[derive(Debug)]
struct TypingPattern {
    pub name: String,
    pub pattern: String,
}

trait TypingPatternGenerator {
    fn generate(&self) -> TypingPattern;
}
```

This way, we can keep track of typos per pattern, and then iterate once we generate
new patterns, to really home in on specific drills.

### Recursive generators

It should be possible to create recursive generators.
Actually I think that might be a bad idea, or I would have to keep a count of the recursion
and I don't feel like fighting the rust borrow checker right now by making generate() mutable.

In general I realize how the effort of dealing with rust's strong checks makes me less willing
to experiment. It makes prototyping harder, in ways that I think are not just related to my 
inexperience with the language.

Of course, the fact that I'm trying to program with a keyboard I'm not familiar with either
and with a baseline of stress from the week doesn't help at all.

### Instantiating generators from a config file

In the configuration file, we want to define generators,
but we also want to specify how to maybe create generator factories? So for example,
method call factories, should we provide them with plugs for the symbol generate, the argument generators?

Or is it better to just define those in code for now.

I think we should keep them in code for now, and refine the factory approach next time.

### Probabilities

Besides just creating a probabilities generator that gets a list of weights + other generator,
we can create a generator that dynamically adjusts weights as we type, going through a list of
different patterns once a certain threshold is reached, randomly inserting patterns (spaced repetition 
style).

### Tracking progress

Each session of the tool should record the typos we made, which patterns were mistyped, which
were practiced, speed, typos, etc...

Besides just using this for metrics and tracking progress, we can feed this into our spaced repetition
algorithm (see next session).

### Spaced repetition

Besides keeping a list of patterns and 

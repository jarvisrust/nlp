[![Build Status](https://travis-ci.org/jarvisrust/nlp.svg)](https://travis-ci.org/jarvisrust/nlp)

# Rust NLP

To use, add this to your `Cargo.toml` file

```
[dependencies.nlp]
git = "https://github.com/jarvisrust/nlp.git"
```

Then, in your project, simply add
```
extern crate nlp;
```

Tokenizer is currently the only working library.

To use it, pass a string to:
```
nlp::tokenizer::tokenize(String);
```
and it will return a `Vec<String>` of all the separate tokens



Still to add for tokenizer:

1. Tokenizing for things like `n't`


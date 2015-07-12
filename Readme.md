[![Build Status](https://travis-ci.org/jarvisrust/nlp.svg)](https://travis-ci.org/jarvisrust/nlp)

# Rust NLP

## Usage 

To use, add this to your `Cargo.toml` file

```
[dependencies.nlp]
git = "https://github.com/jarvisrust/nlp.git"
```

Then, in your project, simply add
```
extern crate nlp;
```

Tokenizer and ngram are currently the only working libraries.


## Tokenizer 

To use the Tokenizer, pass a string to:
```
nlp::tokenizer::tokenize(String);
```
and it will return a `Vec<String>` of all the separate tokens

Other tokenizing features:

```
pub type Token = String;
pub type TokenList = Vec<Token>;

nlp::tokenizer::split_on_spaces(input: TokenList);
nlp::tokenizer::split_on_hyphen(input: TokenList);
nlp::tokenizer::split_on_punctuation(input: TokenList);
```

## NGram

To create ngrams, use:
```
nlp::ngram::create_from_string(n, string);
```
where `n` is the number of tokens per ngram and `string` is a String to be used to generate ngrams.

You can also add padding to the front (*) and a STOP symbol at the back with the following:
```
let ng = nlp::ngram::create_from_string(3, "Any string you want");

ng.add_front_padding();
ng.add_stop();
``

## TODO 

Still to add for tokenizer:

1. Tokenizing for things like `n't`

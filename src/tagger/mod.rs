/**
   Copyright 2015 W. Max Lees

   This file is part of Jarvis OS.

   Jarvis OS is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   Jarvis OS is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with Jarvis OS.  If not, see <http://www.gnu.org/licenses/>.
*/

extern crate nalgebra as na;

//pub mod brown_tags;
//pub mod penn_treebank_tags;
pub mod tags;

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

use ngram;
use tokenizer;

pub struct PoSTagger {
    q: tags::TagProbabilityQ,
    e: tags::TagProbabilityE,

    //parse_tag: fn(&str) -> tags::TagPair<T>,
}

impl PoSTagger {
    fn parse_tag(input: String) -> tags::TagPair {
        // Split the input string
        let mut parts = input.split("/");

        // Get the word and tag
        let new_word = parts.next().unwrap().to_string();
        let new_tag = parts.next().unwrap().to_string();

        (new_word, new_tag)
    }

    pub fn new() -> PoSTagger {
        let new_tagger = PoSTagger{
            q: tags::TagProbabilityQ::new(),
            e: tags::TagProbabilityE::new(),
        };

        new_tagger
    }

    pub fn learn_from_corpus(&mut self, filename: String) {
        let mut training_e = tags::TagProbabilityE::new();
        let mut training_q = tags::TagProbabilityQ::new();

        let mut f = match File::open(filename) {
            Ok(file) => file,
            Err(_) => return,
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => {},
            Err(_) => return,
        }

        let words = tokenizer::tokenize(s.clone());
        for word in words {
            PoSTagger::calculate_probability_e(word, &mut training_e);
        }

        let tags = ngram::NGramSet::create_from_string(3, s.clone());


    }

    fn calculate_probability_e(input: String, e: &mut tags::TagProbabilityE) {
        let pair = PoSTagger::parse_tag(input);

        let key = tags::generate_key_from_pair(pair);

        let counter = e.entry(key).or_insert(1.0);
        *counter += 1.0;
    }
}

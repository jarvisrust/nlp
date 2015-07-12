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
use std::fs;
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

        let size: usize = parts.clone().count();

        if size < 2 {
            return (parts.next().unwrap().to_string(), "".to_string());
        }

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

    pub fn learn_from_corpus(&mut self, directory: String) {
        let mut training_e = tags::TagProbabilityE::new();
        let mut training_q = tags::TagProbabilityQ::new();

        for file in PoSTagger::get_files_in_dir(directory) {

            println!("Processing File: {:?}", file);

            let mut f = match File::open(file) {
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
                PoSTagger::count_occurences_for_e(word, &mut training_e);
            }

            let tags = ngram::NGramSet::create_from_string(3, s.clone());
            for ngram in tags.get_vec() {
                PoSTagger::count_occurences_for_q(ngram.clone(), &mut training_q);
            }
        }

        for (word, hash) in training_e {
            let total = match hash.get("total") {
                None => continue,
                Some(val) => *val,
            };

            for (tag, amt) in hash {
                if tag == "total".to_string() {
                    continue;
                }
                let word_hash = self.e.entry(word.clone()).or_insert(HashMap::new());
                word_hash.insert(tag, amt/total);
            }
        }

        for (two_tag, hash) in training_q {
            let total = match hash.get("total") {
                None => continue,
                Some(val) => *val,
            };

            for (tag, amt) in hash {
                if tag == "total".to_string() {
                    continue;
                }
                let tag_hash = self.q.entry(two_tag.clone()).or_insert(HashMap::new());
                tag_hash.insert(tag, amt/total);
            }
        }
    }

    fn get_files_in_dir(directory: String) -> Vec<String> {
        let mut result = Vec::<String>::new();

        let files = match fs::read_dir(directory) {
            Ok(some) => some,
            Err(e) => {
                println!("Error reading directory: {}", e);
                return result;
            },
        };

        for file in files {
            let filename = match file {
                Ok(path) => path,
                Err(e) => {
                    println!("Error converting filename: {}", e);
                    return result;
                }
            };

            if filename.file_name().to_str().unwrap() == ".DS_Store" {
                println!("Skipping .DS_Store");
                continue;
            }

            result.push(filename.path().to_str().unwrap().to_string());
        }

        result
    }

    fn count_occurences_for_e(input: String, e: &mut tags::TagProbabilityE) {
        let pair = PoSTagger::parse_tag(input);

        match pair {
            (word, tag) => {
                let mut word_hash = e.entry(word).or_insert(HashMap::new());
                {
                    let total = word_hash.entry("total".to_string()).or_insert(0.0);
                    *total += 1.0;
                }
                let counter = word_hash.entry(tag).or_insert(0.0);
                *counter += 1.0;
            },
        }
    }

    fn count_occurences_for_q(input: ngram::NGram, q: &mut tags::TagProbabilityQ) {
        let mut tags = Vec::<String>::new();
        for gram in input {
            let (_, tag) = PoSTagger::parse_tag(gram);
            tags.push(tag);
        }

        let mut key = String::new();
        let separator = "```".to_string();
        key.push_str(&tags.get(0).unwrap()[..]);
        key.push_str(&separator[..]);
        key.push_str(&tags.get(1).unwrap()[..]);

        let mut tag_hash = q.entry(key.clone()).or_insert(HashMap::new());
        {
            let total = tag_hash.entry("total".to_string()).or_insert(0.0);
            *total += 1.0;
        }

        let counter = tag_hash.entry(tags.get(2).unwrap().clone()).or_insert(0.0);
        *counter += 1.0;
    }
}

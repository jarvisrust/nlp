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

pub mod brown_tags;
pub mod penn_treebank_tags;
pub mod tags;

use std::io::prelude::*;
use std::fs::File;

use ngram;

const N: usize = 86;
const M: usize = 1100000;

// N = 86       Number of tags
// M = 1.1M     Number of words in English language

/*
A function for parsing a word in a tagged
corpus. These come in the form "word/tag" 
*/

pub struct PoSTagger<T> {
    pi: na::DVec<f32>,
    a: na::DMat<f32>,
    b: na::DMat<f32>,

    tag_function: fn(&str) -> tags::TagPair<T>,
}

impl <T> PoSTagger<T> {
    pub fn new(f: fn(&str) -> tags::TagPair<T>) -> PoSTagger<T> {
        let max_iter: u16 = 10000;
        let iters: u16 = 0;

        let new_tagger = PoSTagger::<T>{
            pi: na::DVec::<f32>::from_elem(N, (1 as f32/N as f32)),
            a: na::DMat::<f32>::from_elem(N, N, (1 as f32/N as f32)),
            b: na::DMat::<f32>::from_elem(N, M, (1 as f32/M as f32)),

            tag_function: f,

        };

        new_tagger
    }

    pub fn learn_from_corpus(filename: String) {
        let mut f = match File::open(filename) {
            Ok(file) => file,
            Err(_) => return,
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => {},
            Err(_) => return,
        }

        let tags = ngram::NGramSet::create_from_string(3, s);
    }
}
